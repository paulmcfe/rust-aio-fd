use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use the browser's console.log function
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // Use the browser's high-resolution timer: performance.now()
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

// A simple (non-optimized) prime check.
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

#[wasm_bindgen]
pub fn count_primes(limit: u32) {
    let start = now();

    let mut count = 0;
    for n in 2..=limit {
        if is_prime(n) {
            count += 1;
        }
    }

    let end = now();
    let elapsed = end - start;

    let message = format!(
        "Found {count} primes up to {limit} in {elapsed:.3} ms (computed in Rust, logged from console.log)."
    );

    log(&message);
}
