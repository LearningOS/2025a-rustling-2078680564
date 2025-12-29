// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Wrapper<T> {
    value: T, // 把固定的 u32 替换为泛型 T
}

// 为泛型结构体 Wrapper<T> 实现方法，注意 impl 后要声明泛型参数 T
impl<T> Wrapper<T> {
    // 方法参数和返回值中的 u32 都替换为 T
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}