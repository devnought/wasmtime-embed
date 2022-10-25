use std::{
    ffi::{CStr, CString},
    fs, str,
};

extern "C" {
    fn host_print(ptr: i32);
}

fn print<S>(message: S)
where
    S: AsRef<str>,
{
    let message_ref = message.as_ref();
    let ffi_message = CString::new(message_ref).unwrap();

    unsafe {
        host_print(ffi_message.as_ptr() as i32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn run(value: i32, ptr_host: i32) -> i32 {
    let passed_string = CStr::from_ptr(ptr_host as *const i8).to_str().unwrap();

    run_safe(value, passed_string)
}

fn run_safe(value: i32, message: &str) -> i32 {
    print(format!("Passed string: {message}"));

    let marks = std::iter::repeat("!").take(value as usize);
    let test = ["Hey there"].into_iter().chain(marks).collect::<String>();

    print(test);

    let other = format!("Some value: {value}");
    print(other);

    if let Ok(d) = fs::read_to_string("/LICENSE") {
        let message = format!("Read {} characters from LICENSE", d.len());
        print(message);
    }

    value * value
}