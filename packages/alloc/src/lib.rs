#![allow(non_upper_case_globals)]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "lol_alloc", target_arch = "wasm32"))]
#[global_allocator]
static ALLOCATOR: lol_alloc::LockedAllocator<lol_alloc::FreeListAllocator> = lol_alloc::LockedAllocator::new(lol_alloc::FreeListAllocator::new());

#[cfg(all(feature = "talc", target_arch = "wasm32"))]
#[global_allocator]
static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };

#[wasm_bindgen]
extern "C" {
    #[no_mangle]
    static performance: Performance;
}

#[wasm_bindgen]
extern "C" {
    type Performance;

    #[wasm_bindgen(method, structural, js_class = "Performance", js_name = now)]
    pub fn now(this: &Performance) -> f64;
}

#[wasm_bindgen]
pub struct Benchmark {
    data: Vec<f64>,
}

#[wasm_bindgen]
impl Benchmark {
    pub fn data_at(&self, at: usize) -> f64 {
        self.data[at]
    }
}

#[wasm_bindgen]
pub fn abc(input: Vec<u8>) -> Benchmark {
    let mut result = Vec::new();
    let mut dummy_val = 0u64;
    for i in [1, 4, 16/* , 64, 256, 1024, 4096, 16384, 65536*/] {
        let _kk = vec![1u8; 400000000];
        let start = performance.now();
        let iterations = 10000000 / (i / 64).max(1);

        // a persistent growable vector to lessen the chance that the
        // allocator is just using the same spot to allocate and free
        // the iteration vector
        let mut scratch = Vec::new();
        for _ in 0..iterations {
            // 
            let mut res = vec![dummy_val; i];
            res.remove(res.len() / 2);
            dummy_val = dummy_val.wrapping_add(res.len() as u64 + 1);
            scratch.push((dummy_val % 256) as u8);
        }

        result.push(performance.now() - start)
    }

    Benchmark { data: result }
}

#[wasm_bindgen]
pub fn allocation(corpus: &str, iterations: usize) -> usize {
    let mut data = Vec::new();

    for _ in 0..iterations {
        let mut new_data = Vec::new();
        for _ in 0..5 {
            new_data.push(String::from(corpus));
        }
        data.push(new_data);
    }

    data.len()
}
