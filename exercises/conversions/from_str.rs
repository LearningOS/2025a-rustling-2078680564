// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 步骤1：空字符串返回 Empty 错误
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        // 步骤2：按逗号分割字符串并收集到 Vec 中
        let parts: Vec<&str> = s.split(',').collect();

        // 步骤3：分割后必须恰好 2 个字段，否则返回 BadLen
        if parts.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        // 步骤4：提取名字字段，检查是否为空
        let name_part = parts[0];
        if name_part.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        // 步骤5：解析年龄字段，失败则包装为 ParseInt 错误
        let age = parts[1]
            .parse::<usize>()
            .map_err(ParsePersonError::ParseInt)?;

        // 所有条件满足，返回成功的 Person 实例
        Ok(Person {
            name: name_part.to_string(),
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
        assert_eq!(",".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert_eq!(",one".parse::<Person>(), Err(ParsePersonError::NoName));
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