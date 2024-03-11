use alloc::vec::Vec;

use value_stack::Stack;

use crate::{Result, ValidationInfo};
use crate::core::indices::{FuncIdx, LocalIdx};
use crate::core::reader::{WasmReadable, WasmReader};
use crate::core::reader::types::{FuncType, NumType, ValType};
use crate::execution::assert_validated::UnwrapValidatedExt;
use crate::execution::locals::Locals;
use crate::execution::store::{FuncInst, Store};
use crate::execution::value::Value;
use crate::validation::code::read_declared_locals;
use crate::value::{InteropValue, InteropValueList};

// TODO
pub(crate) mod assert_validated;
pub(crate) mod label;
pub(crate) mod locals;
pub(crate) mod value;
pub mod value_stack;
pub(crate) mod store;

pub struct RuntimeInstance<'b> {
    wasm_bytecode: &'b [u8],
    types: Vec<FuncType>,
    store: Store,
}

impl<'b> RuntimeInstance<'b> {
    pub fn new(validation_info: &'_ ValidationInfo<'b>) -> Result<Self> {
        trace!("Starting instantiation of bytecode");

        let store = Self::init_store(&validation_info);

        // TODO execute start function

        Ok(RuntimeInstance {
            wasm_bytecode: validation_info.wasm,
            types: validation_info.types.clone(),
            store,
        })
    }
    /// Can only invoke functions with signature `[t1] -> [t2]` as of now.
    pub fn invoke_func<Param: InteropValueList, Returns: InteropValueList>(
        &mut self,
        func_idx: FuncIdx,
        mut param: Param,
    ) -> Returns {
        let func_inst = self.store.funcs.get(func_idx).expect("valid FuncIdx");
        let func_ty = self.types.get(func_inst.ty).unwrap_validated();

        if &func_ty.params.valtypes != Param::TYS {
            panic!("Invalid `Param` generics");
        }
        if &func_ty.returns.valtypes != Returns::TYS {
            panic!("Invalid `Returns` generics");
        }
        // TODO check if parameters and return types match the ones in `func_ty`


        let mut locals = Locals::new(param.into_values().into_iter(), func_inst.locals.iter().cloned());
        let mut stack = Stack::new();

        /// Start reading the function's instructions
        let mut wasm = WasmReader::new(self.wasm_bytecode);
        wasm.move_start_to(func_inst.code_expr);

        loop {
            match wasm.read_u8().unwrap_validated() {
                // end
                0x0B => {
                    break;
                }
                // local.get: [] -> [t]
                0x20 => {
                    let local_idx = wasm.read_var_u32().unwrap_validated() as LocalIdx;
                    let local = locals.get(local_idx);
                    trace!("Instruction: local.get [] -> [{local:?}]");
                    stack.push_value(local.clone());
                }
                // local.set [t] -> []
                0x21 => {
                    let local_idx = wasm.read_var_u32().unwrap_validated() as LocalIdx;
                    let local = locals.get_mut(local_idx);
                    let value = stack.pop_value(local.to_ty());
                    trace!("Instruction: local.set [{local:?}] -> []");
                    *local = value;
                }
                // i32.add: [i32 i32] -> [i32]
                0x6A => {
                    let v1: i32 = stack.pop_value(ValType::NumType(NumType::I32)).into();
                    let v2: i32 = stack.pop_value(ValType::NumType(NumType::I32)).into();
                    let res = v1.wrapping_add(v2);

                    trace!("Instruction: i32.add [{v1} {v2}] -> [{res}]");
                    stack.push_value(res.into());
                }
                // i32.const: [] -> [i32]
                0x41 => {
                    let constant = wasm.read_var_i32().unwrap_validated();
                    trace!("Instruction: i32.const [] -> [{constant}]");
                    stack.push_value(constant.into());
                }
                other => {
                    trace!("Unknown instruction {other:#x}, skipping..");
                }
            }
        }

        let mut values = Returns::TYS
            .iter()
            .map(|ty| stack.pop_value(ty.clone()))
            .collect::<Vec<Value>>();
        // Values are reversed because they were popped from stack one-by-one. Now reverse them back
        let reversed_values = values.into_iter().rev();
        let ret = Returns::from_values(reversed_values);
        debug!("Successfully invoked function");
        ret
    }

    fn init_store(validation_info: &ValidationInfo) -> Store {
        let function_instances: Vec<FuncInst> = {
            let mut wasm_reader = WasmReader::new(validation_info.wasm);

            let functions = validation_info.functions.iter();
            let func_blocks = validation_info.func_blocks.iter();

            functions.zip(func_blocks).map(|(ty, func)| {
                wasm_reader.move_start_to(*func);

                let (locals, bytes_read) = wasm_reader.measure_num_read_bytes(|wasm| read_declared_locals(wasm)).unwrap_validated();

                let code_expr = wasm_reader.make_span(func.len() - bytes_read);

                FuncInst {
                    ty: *ty,
                    locals,
                    code_expr,
                }
            }).collect()
        };

        Store {
            funcs: function_instances,
            mems: Vec::new(),
        }
    }
}