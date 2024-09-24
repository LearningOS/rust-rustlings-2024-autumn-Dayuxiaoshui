// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
macro_rules! my_macro {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

fn main() {
    my_macro!("Hello from my macro!");
}
