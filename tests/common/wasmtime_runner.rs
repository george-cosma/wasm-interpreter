use crate::common::{Runner, UniversalParams, UniversalResults};

pub struct WASMTimeRunner<T> {
    instance: wasmtime::Instance,
    store: wasmtime::Store<T>,
    func_name: Option<String>,
}

impl<T> WASMTimeRunner<T> {
    pub fn new(wat: &str, initial_store: T) -> wasmtime::Result<Self> {
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, wat)?;
        let linker = wasmtime::Linker::new(&engine);

        let mut store = wasmtime::Store::new(&engine, initial_store);
        let instance = linker.instantiate(&mut store, &module)?;

        Ok(WASMTimeRunner { instance, store, func_name: None })
    }
}

impl<T> Runner for WASMTimeRunner<T> {
    fn execute<Params: UniversalParams, Output: UniversalResults>(
        &mut self,
        _func_id: usize,
        func_name: &str,
        params: Params,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        let function = self
        .instance
        .get_typed_func::<Params, Output>(&mut self.store, func_name)?;

        Ok(function.call(&mut self.store, params)?)
    }
    
    fn set_function(&mut self, _func_id: usize, func_name: &str) {
        self.func_name = Some(func_name.to_string());
    }
    
    fn execute_fn<Params: UniversalParams, Output: UniversalResults>(
        &mut self,
        params: Params,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        if let Some(func_name) = &self.func_name {
            let function = self
                .instance
                .get_typed_func::<Params, Output>(&mut self.store, func_name)?;

            Ok(function.call(&mut self.store, params)?)
        } else {
            Err("No function set to run".into())
        }
    }
}
