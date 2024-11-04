mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
struct Mathematician{
    a: i32,
    b: i32
}
#[wasm_bindgen]
impl Mathematician{
    pub fn new(a:i32, b:i32)->Self{
        Self{a, b}
    }
    pub fn add(&self)-> i32{
        let result = self.a + self.b;
        result
    }
    pub fn sub(&self) -> i32{
        let result = self.a - self.b;
        result
    }
    pub fn mul(&self) -> i32{
        let result = self.a * self.b;
        result
    }
    pub fn div(&self) ->f32{
        let result : f32 = self.a as f32 / self.b as f32;
        result
    }
}