// #[macro_export]
// macro_rules! poly_test {
//     (($runner_name:ident, $assert:expr), $($runner:expr),+ ) => {
//         $(
//             {
//                 let mut $runner_name = $runner;

//                 $assert
//             }
//         )+
//     };
// }

#[macro_export]
macro_rules! test_init {
    ($wat:expr, ($wasmtime:ident, $in_place:ident, $wasmi:ident)) => {
        let mut $wasmtime = {
            let engine = wasmtime::Engine::default();
            let module =
                wasmtime::Module::new(&engine, $wat).expect("wasmtime `Module::new` failed");
            let linker = wasmtime::Linker::new(&engine);

            // let mut store = wasmtime::Store::new(&engine, initial_store);
            let mut store = wasmtime::Store::new(&engine, ());
            let instance = linker
                .instantiate(&mut store, &module)
                .expect("wasmtime `linker.instantiate` failed");

            (instance, store)
        };

        let wasm_bytes = wat::parse_str($wat).unwrap();
        let validation_info = validate(&wasm_bytes).expect("validation failed");
        let mut $in_place = RuntimeInstance::new(&validation_info).expect("instantiation failed");

        let mut $wasmi = {
            let engine = wasmi::Engine::default();
            let module = wasmi::Module::new(&engine, &mut &wasm_bytes[..])
                .expect("Failed module initiatlization - wasmi");
            // let mut store = Store::new(&engine, initial_store);
            let mut store = wasmi::Store::new(&engine, ());

            let linker = wasmi::Linker::new(&engine);
            let instance = linker
                .instantiate(&mut store, &module)
                .expect("failed wasmi instantiation")
                .start(&mut store)
                .expect("failed wasmi start");

            (instance, store)
        };
    };
}

#[macro_export]
macro_rules! test_run {
    (($func_id:expr, $func_name:expr), $params:expr, ($wasmtime_result:ident, $in_place_result:ident, $wasmi_result:ident) ,($wasmtime:expr, $in_place:expr, $wasmi:expr)) => {
        $wasmtime_result = {
            let function = $wasmtime
                .0
                .get_typed_func(&mut $wasmtime.1, $func_name)
                .expect("failed to find wasm function");

            function
                .call(&mut $wasmtime.1, $params)
                .expect("function execution failed")
        };

        $in_place_result = $in_place.invoke_func($func_id, $params);

        $wasmi_result = {
            let function = $wasmi
                .0
                .get_typed_func(&mut $wasmi.1, $func_name)
                .expect("failed to find wasm function");

            function
                .call(&mut $wasmi.1, $params)
                .expect("function execution failed")
        };
    };
}

#[macro_export]
macro_rules! test_eq {
    (($func_id:expr, $func_name:expr), $params:expr, ($expected:expr, $ex_type:ty) ,($wasmtime:expr, $in_place:expr, $wasmi:expr)) => {{
        let r1: $ex_type;
        let r2: $ex_type;
        let r3: $ex_type;
        test_run!(
            ($func_id, $func_name),
            $params,
            (r1, r2, r3),
            ($wasmtime, $in_place, $wasmi)
        );
        assert_eq!($expected, r1);
        assert_eq!($expected, r2);
        assert_eq!($expected, r3);
    }};
}
