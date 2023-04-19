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
    let (mut store, instance) = instantiate();

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

#[wasm_bindgen]
pub fn test_add_i64() -> String {
    let (mut store, instance) = instantiate();

    let add_i64 = instance
        .exports
        .get_typed_function::<(i64, i64), i64>(&store, "add_i64")
        .expect("should get add_i64 export");

    let pairs = [
        (5, 10),
        (5, -10),
        (i64::MAX - 5, 10),
        (i64::MIN + 5, -10),
        (-5, 10),
        (-5, -10),
        (i64::MAX - 15, 10),
        (i64::MIN + 15, -10),
    ];

    let mut result_text = String::new();
    for (a, b) in pairs {
        let wasm_result = add_i64.call(&mut store, a, b);
        let compare_result = a.wrapping_add(b);

        let wasm_result = match wasm_result {
            Ok(value) => value,
            Err(err) => {
                write!(
                    result_text,
                    "ERR: calling {} + {} should give {}, but it gave error instead:\n{:?}\n\n",
                    a, b, compare_result, err
                )
                .expect("writing to string is infallible");
                continue;
            }
        };

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

#[wasm_bindgen]
pub fn test_is_even_i64(value: i64) -> bool {
    let (mut store, instance) = instantiate();

    let is_even_i64 = instance
        .exports
        .get_typed_function::<i64, u8>(&store, "is_even_i64")
        .expect("should get is_even_i64 export");

    is_even_i64
        .call(&mut store, value)
        .expect("should call is_even_i64")
        != 0
}

#[wasm_bindgen]
pub fn test_combine_to_i64(upper: i32, lower: i32) -> i64 {
    console_error_panic_hook::set_once();

    let upper = (upper as i64) << 32;
    let lower = (lower as i64) & 0xff_ff_ff_ff;
    return upper | lower;

    // let (mut store, instance) = instantiate();

    // let combine_to_i64 = instance
    //     .exports
    //     .get_typed_function::<(i32, i32), i64>(&store, "combine_to_i64")
    //     .expect("should get combine_to_i64 export");

    // combine_to_i64
    //     .call(&mut store, upper, lower)
    //     .expect("should call combine_to_i64")
}

fn instantiate() -> (Store, Instance) {
    console_error_panic_hook::set_once();

    // let mut store = Store::new()
    let mut store = Store::default();

    let module = Module::new(&store, PLUGIN_BYTES).expect("should load module");

    let import_object = imports! {};
    let instance =
        Instance::new(&mut store, &module, &import_object).expect("should create instance");

    (store, instance)
}
