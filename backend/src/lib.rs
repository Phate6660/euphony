use rodio::{Decoder, OutputStream};
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern fn play(path: *const c_char) {
    let path = unsafe { CStr::from_ptr(path).to_str().unwrap() };
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    println!("Playing: {}", path);
    let file = std::fs::File::open(path).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
