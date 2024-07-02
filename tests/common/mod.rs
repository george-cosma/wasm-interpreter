pub(crate) mod wasmtime_runner;
pub(crate) mod wasm_interpreter_runner;

extern crate wasm;
use std::error::Error;
use std::fmt::Debug;
use wasm::value::InteropValueList;
use wasmtime::{WasmParams, WasmResults, WasmTy};


pub trait UniversalParams : wasmtime::WasmParams + wasm::value::InteropValueList {}
impl<T> UniversalParams for T where T : wasmtime::WasmParams + wasm::value::InteropValueList {}

pub trait UniversalResults : wasmtime::WasmResults + wasm::value::InteropValueList {}
impl<T> UniversalResults for T where T : wasmtime::WasmResults + wasm::value::InteropValueList {}

// TODO: also add wasmi?
pub trait Runner {
    /// Execute a function with the given parameters and return the result
    fn execute<Params: UniversalParams, Output: UniversalResults>(
        &mut self,
        func_id: usize,
        func_name: &str,
        params: Params,
    ) -> Result<Output, Box<dyn Error>>;

    /// To help simply the workflow, a runner can be programmed to run a specific function
    /// over and over again. This function sets the function to be run.
    fn set_function(&mut self, func_id: usize, func_name: &str);

    /// Execute the function set by `set_function` with the given parameters and return the result
    fn execute_fn<Params: UniversalParams, Output: UniversalResults>(
        &mut self,
        params: Params,
    ) -> Result<Output, Box<dyn Error>>;
}

#[macro_export]
macro_rules! poly_test {
    (($runner_name:ident, $assert:expr), $($runner:expr),+ ) => {
        $(
            {
                let mut $runner_name = $runner;
                
                $assert
            }
        )+
    };
}

// pub fn poly_test_once<Params, Output>(
//     params: Params,
//     expected_result: Output,
//     function_id: usize,
//     function_name: &str,
//     runners: &mut [impl Runner],
// ) where
//     Params: UniversalParams + Clone,
//     Output: UniversalResults + Debug + PartialEq,
// {
//     for runner in runners {
//         let output = runner
//             .execute::<Params, Output>(function_id, function_name, params.clone())
//             .expect("Runner execution failed");

//         assert_eq!(output, expected_result);
//     }
// }
