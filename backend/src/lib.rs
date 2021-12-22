use rodio::{Decoder, OutputStream};
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
/// # Safety
/// This function is unsafe because it may dereference the pointer to the string passed to it.
pub unsafe extern fn play(path: *const c_char) {
    let path = CStr::from_ptr(path).to_str().unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    println!("Playing: {}", path);
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
