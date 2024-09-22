use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 定义 ParsePersonError 枚举，用于表示解析错误
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,        // 输入为空
    BadLen,       // 输入的元素数量不正确
    NoName,       // 名字部分为空
    ParseInt(ParseIntError), // 年龄解析错误
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 如果字符串为空，返回错误
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();

        // 如果分割后的长度不等于 2，返回错误
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // 获取名字并去除空格
        let name = parts[0].trim();
        // 如果名字为空，返回错误
        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 尝试解析年龄部分
        let age = parts[1].trim().parse::<usize>().map_err(ParsePersonError::ParseInt)?;

        // 返回成功的 Person 实例
        Ok(Person {
            name: name.to_string(),
            age,
        })
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
