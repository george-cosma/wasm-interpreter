use crate::common::{Runner, UniversalParams, UniversalResults};

extern crate wasm;

pub struct WASMInterpreterRunner<'a> {
    instance: wasm::RuntimeInstance<'a>,
    func_id: Option<usize>,
}

impl Runner for WASMInterpreterRunner<'_> {
    fn execute<Params: UniversalParams, Output: UniversalResults>(
        &mut self,
        func_id: usize,
        _func_name: &str,
        params: Params,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        Ok(self.instance.invoke_func::<Params, Output>(func_id, params))
    }
    
    fn set_function(&mut self, func_id: usize, _func_name: &str) {
        self.func_id = Some(func_id);
    }
    
    fn execute_fn<Params: UniversalParams, Output: UniversalResults>(
        &mut self,
        params: Params,
    ) -> Result<Output, Box<dyn std::error::Error>> {
        if let Some(func_id) = self.func_id {
            Ok(self.instance.invoke_func::<Params, Output>(func_id, params))
        } else {
            Err("No function set to run".into())
        }
    }
}

impl<'a> From<wasm::RuntimeInstance<'a>> for WASMInterpreterRunner<'a> {
    fn from(instance: wasm::RuntimeInstance<'a>) -> Self {
        WASMInterpreterRunner {
            instance,
            func_id: None,
        }
    }
}
