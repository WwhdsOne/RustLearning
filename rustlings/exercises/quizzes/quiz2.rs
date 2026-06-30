//这是针对以下部分的测验：
//-字符串
//-向量
//-移动语义
//-模块
//-枚举
//
//让我们以函数的形式构建一个小机器。作为输入，我们将
//给出字符串和命令的列表。这些命令决定执行什么操作
//将应用于字符串。它可以是：
//-大写字符串
//-修剪字符串
//-将“bar”附加到字符串指定的次数
//
//其具体形式为：
//-输入将是 2 长度元组的向量，
//第一个元素是字符串，第二个元素是命令。
//-输出元素将是一个字符串向量。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for (s, cmd) in input {
            let result = match cmd {
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_string(),
                Command::Append(size) => {
                    let mut origin = s;
                    for _ in 0..size {
                        origin.push_str("bar");
                    }
                    origin
                }
            };
            v.push(result);
        }
        v
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
