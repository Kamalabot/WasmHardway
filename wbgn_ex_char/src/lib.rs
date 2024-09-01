use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str); 
    
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32
}

#[wasm_bindgen]
impl Counter {
    // creating new counter
    pub fn new(key: char, count: i32) -> Counter {
        log(&format!("Counter::new({}, {})", key, count));
        Counter { key, count }
    }
    // returning the key and logging it
    pub fn key(&self) -> char {
        log("Counter.key");
        self.key
    }

    // returning the count and logging it
    pub fn count(&self) -> i32 {
        log("Counter.count");
        self.count
    }

    // only incrementing count 
    pub fn increment(&mut self) {
        log("Counter.increment");
        self.count += 1
    }

    // updating the current key
    pub fn update_key(&mut self, key: char) {
        self.key = key
    }

}
