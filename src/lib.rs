use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;
use web_sys::{console, Headers, Request, RequestInit, RequestMode, RequestRedirect, Response};

use serde_json::to_string;
use serde_json::{to_value, Value};
use web_sys::js_sys::Math;

#[wasm_bindgen]
extern "C" {
    // pub fn alert(s: &str);
    // #[wasm_bindgen(js_namespace = console)]

}

#[wasm_bindgen]
pub fn generate_lotto_numbers() -> JsValue {
    let mut numbers: Vec<u32> = (1..=45).collect(); // 1부터 45까지의 숫자를 생성
    let mut selected_numbers: Vec<u32> = Vec::new();

    // 6개의 무작위 번호 선택
    while selected_numbers.len() < 6 {
        // JavaScript의 Math.random()을 사용하여 1에서 45 사이의 무작위 숫자 생성
        let random_index = (Math::random() * 45.0).floor() as usize; // 0부터 44 사이의 인덱스
        let number = numbers[random_index];

        // 중복되지 않는 번호만 추가
        if !selected_numbers.contains(&number) {
            selected_numbers.push(number);
        }
    }

    // 선택된 번호를 정렬하여 반환
    selected_numbers.sort();

    // JSON 문자열로 변환 후 JsValue로 변환
    let json_string = serde_json::to_string(&selected_numbers).unwrap();
    JsValue::from_str(&json_string)
    // JsValue::from_serde(&selected_numbers).unwrap() // 결과를 JsValue로 변환하여 반환
}

#[wasm_bindgen]
pub fn generate_lotto_numbers_wasm() -> JsValue {
    generate_lotto_numbers() // JSValue 반환
}
#[wasm_bindgen]
pub fn generate_pension_numbers() -> JsValue {
    let mut numbers = Vec::new();
    
    // 조 번호 생성 (1-7)
    let group = (Math::random() * 7.0).floor() as u32 + 1;
    numbers.push(group);
    
    // 연금복권 6자리 숫자 생성
    // 처음 5자리는 1-9 사이의 숫자
    for _ in 0..5 {
        let number = (Math::random() * 9.0).floor() as u32 + 1;
        numbers.push(number);
    }
    
    // 마지막 자리는 1 또는 2
    let last_number = (Math::random() * 2.0).floor() as u32 + 1;
    numbers.push(last_number);

    // JSON 문자열로 변환 후 JsValue로 변환
    let json_string = serde_json::to_string(&numbers).unwrap();
    JsValue::from_str(&json_string)
}

#[wasm_bindgen]
pub fn generate_pension_numbers_wasm() -> JsValue {
    generate_pension_numbers()
}
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     let window = window().unwrap();
//     window.alert_with_message(name);
// }
// #[wasm_bindgen]
// pub fn log(name: &str) {
//     console::log_1(&format!("{}", name).into());
// }
