use rodio::{Decoder, OutputStream};
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
/// This is the main entry point for the Rust backend which will handle any unsafe calls,
/// which will then be converted to safe Rust calls and then passed to the Rust backend.
///
/// # Safety
/// This function is unsafe because it may dereference the pointer to the string passed to it.
pub unsafe extern fn main_helper(operation: *const c_char, input: *const c_char) {
    let operation = CStr::from_ptr(operation).to_str().unwrap();
    let input = CStr::from_ptr(input).to_str().unwrap();
    match operation {
        "play" => play(input),
        _ => println!("Unknown operation: {}", operation),
    }
}

fn play(path: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    println!("Playing: {}", path);
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
