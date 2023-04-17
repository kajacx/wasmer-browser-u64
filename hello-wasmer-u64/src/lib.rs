use std::fmt::Write;
use wasm_bindgen::prelude::*;
use wasmer::*;

const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../simple-plugin/target/wasm32-unknown-unknown/debug/simple_plugin.wasm");

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn test_add_i32() -> String {
    let mut store = create_store();
    let instance = create_instance(&mut store);

    let add_i32 = instance
        .exports
        .get_typed_function::<(i32, i32), i32>(&store, "add_i32")
        .expect("should get add_i32 export");

    let pairs = [
        (5, 10),
        (5, -10),
        (i32::MAX - 5, 10),
        (i32::MIN + 5, -10),
        (-5, 10),
        (-5, -10),
        (i32::MAX - 15, 10),
        (i32::MIN + 15, -10),
    ];

    let mut result_text = String::new();
    for (a, b) in pairs {
        let wasm_result = add_i32.call(&mut store, a, b).expect("should call add_i32");
        let compare_result = a.wrapping_add(b);

        if wasm_result == compare_result {
            write!(result_text, "OK: {} + {} = {}\n", a, b, compare_result)
                .expect("writing to string is infallible");
        } else {
            write!(
                result_text,
                "ERR: {} + {} should be {}, but got {} instead\n",
                a, b, compare_result, wasm_result
            )
            .expect("writing to string is infallible");
        }
    }
    result_text
}

fn create_store() -> Store {
    Store::new()
}

fn create_instance(store: &mut Store) -> Instance {
    let module = Module::new(&store, PLUGIN_BYTES).expect("should load module");

    let import_object = imports! {};
    Instance::new(store, &module, &import_object).expect("should create instance")
}
