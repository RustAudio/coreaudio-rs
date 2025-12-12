mod feedback;

#[no_mangle]
pub extern "C" fn rust_tvos_main() {
    feedback::run_example().unwrap();
}
