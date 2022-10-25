use std::{
    ffi::{c_char, CStr, CString},
    fs, str,
};

extern "C" {
    fn host_print(ptr: *const c_char);
}

fn print<S>(message: S)
where
    S: AsRef<str>,
{
    let message_ref = message.as_ref();
    let ffi_message = CString::new(message_ref).unwrap();

    unsafe {
        host_print(ffi_message.as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn run(value: i32, ptr_host: i32) -> i32 {
    let passed_string = CStr::from_ptr(ptr_host as *const i8).to_str().unwrap();

    run_safe(value, passed_string)
}

fn run_safe(value: i32, message: &str) -> i32 {
    print(format!("Wasi was passed string: {message}"));

    let marks = std::iter::repeat("!").take(value as usize);
    let test = ["Wasi saysd: Hey there"]
        .into_iter()
        .chain(marks)
        .collect::<String>();

    print(test);

    let other = format!("Wasi has some value: {value}");
    print(other);

    if let Ok(d) = fs::read_to_string("/LICENSE") {
        let message = format!("Wasi read {} characters from LICENSE", d.len());
        print(message);
    }

    value * value
}
