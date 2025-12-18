mod feedback;

#[no_mangle]
pub extern "C" fn rust_apple_main() {
    feedback::run_example().unwrap();
}
