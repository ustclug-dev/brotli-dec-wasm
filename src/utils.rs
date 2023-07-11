/// To get better error messages if our code ever panics
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}
