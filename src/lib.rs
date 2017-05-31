pub mod bridge;
use std::process;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_Resume() {
    println!("_Unwind_Resume from bwapi-sys");
    process::abort();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn _Unwind_RaiseException() {
    println!("_Unwind_RaiseException from bwapi-sys");
    process::abort();
}
