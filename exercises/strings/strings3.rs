fn trim_me(input: &str) -> String {
    input.trim().to_string() // 使用 trim 方法去掉首尾空白
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input) // 使用 format 添加 " world!" 到字符串后面
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons") // 使用 replace 方法替换 "cars" 为 "balloons"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
