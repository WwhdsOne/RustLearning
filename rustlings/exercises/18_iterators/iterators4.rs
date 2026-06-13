fn factorial(num: u64) -> u64 {
    //TODO: 完成此函数以返回 `num` 的阶乘，即
    //定义为 `1 *2 *3 *... *num`。
    //https://en.wikipedia.org/wiki/Factorial
    //
    //不要使用：
    //-提前返回（显式使用 `return` 关键字）
    //尽量不要使用：
    //-命令式循环（for/while）
    //-附加变量
    //对于额外的挑战，不要使用：
    //-递归
    (1..=num).product()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
