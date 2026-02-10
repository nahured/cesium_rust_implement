use wasm_bindgen::prelude::*;
use image::{ImageReader, Pixel, Rgba};


#[wasm_bindgen]
pub fn get_buffer_from_png(path:String){
    let image = ImageReader::open(path)?
    .decode()?;
}