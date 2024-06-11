use hippo_js_parser::{ Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseModule)]
pub fn parse_module(source: String) -> JsValue {
    match Parser::new(&source).parse_module_json() {
        Ok(ast) => JsValue::from_str(&ast),
        Err(err) => JsValue::from(err.to_string()),
    }
}

#[wasm_bindgen(js_name = parseScript)]
pub fn parse_script(source: String) -> JsValue {
    match Parser::new(&source).parse_script_json() {
        Ok(ast) => JsValue::from_str(&ast),
        Err(err) => JsValue::from(err.to_string()),
    }
}
