pub mod bridge;
use std::process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub extern fn _Unwind_Resume() {
    println!("_Unwind_Resume from sys");
    process::abort();
}

#[no_mangle]
pub extern fn _Unwind_RaiseException() {
    println!("_Unwind_Resume from sys");
    process::abort();
}
