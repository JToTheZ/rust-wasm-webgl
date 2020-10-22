extern crate console_error_panic_hook;
extern crate rand;

#[macro_use]
extern crate lazy_static;
use lazy_static::*;

use rand::rngs::{OsRng, StdRng};
use rand::{Rng, SeedableRng};

use std::panic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

mod shader;
use shader::Program;

mod math;
use math::*;

mod app_state;
use app_state::*;

#[wasm_bindgen]
pub fn now() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
}

#[wasm_bindgen]
pub fn generate_from_seed(seed: u32) -> f32 {
    StdRng::seed_from_u64(seed as u64).gen()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

pub fn draw_point_array(context: &WebGlRenderingContext, points: &[f32]) -> Result<(), String> {
    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertex_array = js_sys::Float32Array::view(points);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.draw_arrays(
        WebGlRenderingContext::POINTS,
        0,
        (points.len() / 3) as i32,
    );
    Ok(())
}

pub fn draw_line_array(context: &WebGlRenderingContext, points: &[f32]) -> Result<(), String> {
    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vertex_array = js_sys::Float32Array::view(points);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertex_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(0);

    context.draw_arrays(
        WebGlRenderingContext::LINES,
        0,
        (points.len() / 3) as i32,
    );
    Ok(())
}

pub fn generate_point_cloud(num_points: u32) -> Vec<f32> {
    let mut points = vec![];
    for _ in 0..num_points {
        points.push(1.0 - 2.0 * generate_from_seed(now() as u32));
        points.push(1.0 - 2.0 * generate_from_seed(now() as u32));
        points.push(0.0);
    }
    points
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    setup_listeners()?;
    draw();    
    Ok(())
}

fn draw() -> Result<(), JsValue> {
    set_points(generate_point_cloud(1000));
    let context = setup_gl()?;
    let program = shader::default_shader(&context)?;
    program.use_program(&context)?;

    let points = &app_state().points;
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    log(&format!("{}", points.len()));
    draw_point_array(&context, points)?;
    draw_line_array(&context, points)?;

    Ok(())
}

fn setup_gl() -> Result<WebGlRenderingContext, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width(1920);
    canvas.set_height(1080);
    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;
    Ok(context)
}

fn setup_listeners() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let button = document.get_element_by_id("btn").unwrap();
    let button: web_sys::HtmlButtonElement = button.dyn_into::<web_sys::HtmlButtonElement>()?;
    let listener = Closure::wrap(Box::new(move || {
        
    }) as Box<dyn FnMut()>);

    button.set_onclick(Some(listener.as_ref().unchecked_ref()));
    listener.forget();
    Ok(())
}