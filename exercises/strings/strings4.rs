// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // &str 字符串切片
    string("red".to_string()); // String 类型
    string(String::from("hi")); // String 类型
    string("rust is fun!".to_owned()); // String 类型
    string("nice weather".into()); // String 类型
    string(format!("Interpolation {}", "Station")); // String 类型
    string_slice(&String::from("abc")[0..1]); // &str 字符串切片
    string_slice("  hello there ".trim()); // &str 字符串切片
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // String 类型
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // String 类型
}
