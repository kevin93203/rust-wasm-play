use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add2numbers(n1:i32, n2:i32) -> i32 {
    n1 + n2
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    let mut hash_map = HashMap::new();
    fib_helper(n, &mut hash_map)
}

fn fib_helper(n: i32, hash_map: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&value) = hash_map.get(&n) {
        return value;
    }
    
    if n <= 1 {
        return n;
    }
    
    let result = fib_helper(n - 1, hash_map) + fib_helper(n - 2, hash_map);
    hash_map.insert(n, result);
    
    result
}

use js_sys::{Reflect};

#[wasm_bindgen]
pub fn process_js_object(obj: JsValue) -> JsValue {
    let key_value = Reflect::get(&obj, &JsValue::from_str("key")).unwrap();
    // 進一步處理key_value
    key_value
}

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{to_value};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct MyData {
    #[wasm_bindgen(skip)]
    pub key: String,
    pub number: i32,
}

#[wasm_bindgen]
impl MyData {
    #[wasm_bindgen(constructor)]
    pub fn new(key: String, number:i32) -> MyData {
        MyData { key, number }
    }
}

#[wasm_bindgen]
pub fn process_data(json_str: &str) -> JsValue {
    // 解析JSON字符串
    let data: MyData = serde_json::from_str(json_str).unwrap();
    // 將結果轉換回JsValue
    to_value(&data).unwrap()
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn process_js_ary(js_ary: Vec<MyData>) {
    for item in js_ary {
        alert(&format!("{}:{}",item.key, item.number));
    }
}

