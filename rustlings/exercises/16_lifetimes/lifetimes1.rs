//Rust 编译器需要知道如何检查提供的引用是否是
//有效，这样它可以让程序员知道引用是否有风险
//在使用之前超出范围。请记住，参考文献是借用和做的
//不拥有自己的数据。如果他们的主人超出范围怎么办？

//TODO：通过更新函数签名来修复编译器错误。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
