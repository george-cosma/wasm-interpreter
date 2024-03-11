use alloc::vec::Vec;

use crate::core::indices::{FuncIdx, TypeIdx};
use crate::core::reader::section_header::{SectionHeader, SectionTy};
use crate::core::reader::span::Span;
use crate::core::reader::types::export::Export;
use crate::core::reader::types::import::Import;
use crate::core::reader::types::{FuncType, GlobalType, MemType, TableType};
use crate::core::reader::{WasmReadable, WasmReader};
use crate::{Error, Result};

pub(crate) mod code;

/// Information collected from validating a module.
/// This can be used to create a [crate::RuntimeInstance].
pub struct ValidationInfo<'bytecode> {
    pub(crate) wasm: &'bytecode [u8],
    pub(crate) types: Vec<FuncType>,
    pub(crate) imports: Vec<Import>,
    pub(crate) functions: Vec<TypeIdx>,
    pub(crate) tables: Vec<TableType>,
    pub(crate) memories: Vec<MemType>,
    pub(crate) globals: Vec<GlobalType>,
    pub(crate) exports: Vec<Export>,
    pub(crate) func_blocks: Vec<Span>,
    /// The start function which is automatically executed during instantiation
    pub(crate) start: Option<FuncIdx>,
}

pub fn validate(wasm: &[u8]) -> Result<ValidationInfo> {
    let mut wasm = WasmReader::new(wasm);
    trace!("Starting validation of bytecode");

    trace!("Validating magic value");
    let [0x00, 0x61, 0x73, 0x6d] = wasm.strip_bytes::<4>()? else {
        return Err(Error::InvalidMagic);
    };

    trace!("Validating version number");
    let [0x01, 0x00, 0x00, 0x00] = wasm.strip_bytes::<4>()? else {
        return Err(Error::InvalidVersion);
    };
    debug!("Header ok");

    let mut header = None;
    read_next_header(&mut wasm, &mut header)?;

    let mut skip_section = |wasm: &mut WasmReader, section_header: &mut Option<SectionHeader>| {
        handle_section(wasm, section_header, SectionTy::Custom, |wasm, h| {
            wasm.skip(h.contents.len())
        })
    };

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let types = handle_section(&mut wasm, &mut header, SectionTy::Type, |wasm, _| {
        wasm.read_vec(|wasm| FuncType::read(wasm))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let imports = handle_section(&mut wasm, &mut header, SectionTy::Import, |wasm, _| {
        wasm.read_vec(|wasm| Import::read(wasm))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let functions = handle_section(&mut wasm, &mut header, SectionTy::Function, |wasm, _| {
        wasm.read_vec(|wasm| wasm.read_var_u32().map(|u| u as usize))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let tables = handle_section(&mut wasm, &mut header, SectionTy::Table, |wasm, _| {
        wasm.read_vec(|wasm| TableType::read(wasm))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let memories = handle_section(&mut wasm, &mut header, SectionTy::Memory, |wasm, _| {
        wasm.read_vec(|wasm| MemType::read(wasm))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let globals = handle_section(&mut wasm, &mut header, SectionTy::Global, |wasm, _| {
        wasm.read_vec(|wasm| GlobalType::read(wasm))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let exports = handle_section(&mut wasm, &mut header, SectionTy::Export, |wasm, _| {
        wasm.read_vec(|wasm| Export::read(wasm))
    })?
    .unwrap_or_default();

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let start = handle_section(&mut wasm, &mut header, SectionTy::Start, |wasm, _| {
        wasm.read_var_u32().map(|idx| idx as FuncIdx)
    })?;

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let _: Option<()> = handle_section(&mut wasm, &mut header, SectionTy::Element, |_, _| {
        todo!("element section not yet supported")
    })?;

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let _: Option<()> = handle_section(&mut wasm, &mut header, SectionTy::DataCount, |_, _| {
        todo!("data count section not yet supported")
    })?;

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let func_blocks = handle_section(&mut wasm, &mut header, SectionTy::Code, |wasm, h| {
        code::validate_code_section(wasm, h, &types)
    })?
    .unwrap_or_default();

    assert_eq!(func_blocks.len(), functions.len(), "these should be equal"); // TODO check if this is in the spec

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    let _: Option<()> = handle_section(&mut wasm, &mut header, SectionTy::Data, |_, _| {
        todo!("data section not yet supported")
    })?;

    while let Some(_) = skip_section(&mut wasm, &mut header)? {}

    // All sections should have been handled
    if let Some(header) = header {
        return Err(Error::SectionOutOfOrder(header.ty));
    }

    debug!("Validation was successful");
    Ok(ValidationInfo {
        wasm: wasm.into_inner(),
        types,
        imports,
        functions,
        tables,
        memories,
        globals,
        exports,
        func_blocks,
        start,
    })
}

fn read_next_header(wasm: &mut WasmReader, header: &mut Option<SectionHeader>) -> Result<()> {
    if header.is_none() && wasm.remaining_bytes().len() > 0 {
        *header = Some(SectionHeader::read(wasm)?);
    }
    Ok(())
}

#[inline(always)]
fn handle_section<T, F: FnOnce(&mut WasmReader, SectionHeader) -> Result<T>>(
    wasm: &mut WasmReader,
    header: &mut Option<SectionHeader>,
    section_ty: SectionTy,
    handler: F,
) -> Result<Option<T>> {
    match &header {
        Some(SectionHeader { ty, .. }) if *ty == section_ty => {
            let h = header.take().unwrap();
            trace!("Handling section {:?}", h.ty);
            let ret = handler(wasm, h)?;
            read_next_header(wasm, header)?;
            Ok(Some(ret))
        }
        _ => Ok(None),
    }
}
