use proc_macros::get_third_token;

/// cargo expand --example 03_proc
fn main() {
    get_third_token!({ let a = 1; } { x + 3 } { let b = 4; });
}
