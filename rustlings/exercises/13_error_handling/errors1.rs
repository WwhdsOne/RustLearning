// TODO：如果你向这个函数传入一个空字符串，它会拒绝生成打印在名牌上的文本。
// 如果它能解释问题出在哪里，而不是仅仅返回 `None`，那就更好了。
// 幸运的是，Rust 有一个与 `Option` 类似的构造，可以用来表达错误情况。
// 请修改函数签名和函数体，使其返回 `Result<String, String>` 而不是 `Option<String>`。
fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
