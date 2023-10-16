use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

#[wasm_bindgen]
pub fn decompress(data: &[u8]) -> Vec<u8> {
    lz4_flex::decompress_size_prepended(data).unwrap()
}

#[wasm_bindgen]
pub fn compress(data: &[u8]) -> Vec<u8> {
    lz4_flex::compress_prepend_size(data)
}
