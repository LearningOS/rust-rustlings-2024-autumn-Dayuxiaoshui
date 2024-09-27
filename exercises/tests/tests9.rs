extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // Add `#[no_mangle]` to prevent name mangling
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }

    // Add `#[no_mangle]` and `#[link_name = "my_demo_function"]` for the alias
    #[no_mangle]
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
