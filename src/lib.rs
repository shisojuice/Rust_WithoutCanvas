
use wasm_bindgen::prelude::*;
use std::io::Cursor;
use image::{DynamicImage, ImageFormat, Rgba};
use base64::{engine::general_purpose, Engine as _};
use imageproc::drawing::draw_line_segment_mut;
use serde::{Deserialize, Serialize};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn set_canvas() -> String {
    let canvas = DynamicImage::new_rgba8(320, 320);
    let mut buffer = Cursor::new(Vec::new());
    canvas.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}

#[derive(Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub fn draw_canvas(str_url: String,json_str :String) -> String {
    let points: Vec<Point> = serde_json::from_str(&*json_str).unwrap();
    let comma_pos = str_url.find(',').unwrap();
    let str_base64 :String  =Some(&str_url[comma_pos + 1..]).unwrap().to_string();
    let image_data = general_purpose::STANDARD.decode(str_base64).unwrap();
    let mut img =  image::load_from_memory_with_format(&image_data, ImageFormat::Png).unwrap();
    let color = Rgba([0, 0, 0,255]);
    // 座標をつなげて線を描画
    for i in 0..(points.len() - 1) {
        let start = &points[i];
        let end = &points[i + 1];
        draw_line_segment_mut(
            &mut img,
            (start.x as f32, start.y as f32),
            (end.x as f32, end.y as f32),
            color,
        );
    }
    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let base64_string = general_purpose::STANDARD.encode(buffer.get_ref());
    // データURL形式で返す
    format!("data:image/png;base64,{}", base64_string)
}