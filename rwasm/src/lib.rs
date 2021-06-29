mod utils;

extern crate base64;
extern crate image;
use base64::encode;
use image::{DynamicImage, ImageFormat};
use std::io::{Cursor, Read, Seek, SeekFrom};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    img
}

fn get_image_as_base64(_img: DynamicImage) -> String {
    let mut c = Cursor::new(Vec::new());
    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            );
        }
    };
    c.seek(SeekFrom::Start(0)).unwrap();

    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    let stt = encode(&mut out);
    format!("{}{}", "data:image/png;base64,", stt)
}

fn append_img(img_src: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("img")?;
    val.set_attribute("src", &img_src)?;
    val.set_attribute("style", "height: 200px")?;
    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> Result<(), JsValue> {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    append_img(get_image_as_base64(img))
}
