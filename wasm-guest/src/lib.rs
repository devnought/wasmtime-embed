use std::{slice, str};

extern "C" {
    fn host_print(ptr: i32, len: i32);
}

fn print<S>(message: S)
where
    S: AsRef<str>,
{
    let message_ref = message.as_ref();

    unsafe {
        host_print(message_ref.as_ptr() as i32, message_ref.len() as i32);
    }
}

#[no_mangle]
pub unsafe extern "C" fn run(value: i32, ptr_host: i32, len: i32) -> i32 {
    let passed_slice = slice::from_raw_parts(ptr_host as *const u8, len as usize);
    let passed_string = str::from_utf8_unchecked(passed_slice);

    run_safe(value, passed_string)
}

fn run_safe(value: i32, message: &str) -> i32 {
    print(format!("Passed string: {message}"));

    let marks = std::iter::repeat("!").take(value as usize);
    let test = ["Hey there"].into_iter().chain(marks).collect::<String>();

    print(test);

    let other = format!("Some value: {value}");
    print(other);

    value * value
}
