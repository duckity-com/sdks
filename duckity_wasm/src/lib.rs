use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process(challenge: String) -> Result<String, JsValue> {

    let decoded =
        duckity_core::decode(&challenge).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let solution = duckity_core::solve(&decoded);

    let encoded = duckity_core::encode(&challenge, &solution)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(encoded)
}
