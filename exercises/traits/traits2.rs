trait AppendBar {
    fn append_bar(self) -> Self;
}

// 实现特性 `AppendBar` 针对 `Vec<String>` 类型
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        // 向 vector 添加 "Bar"
        self.push(String::from("Bar"));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
