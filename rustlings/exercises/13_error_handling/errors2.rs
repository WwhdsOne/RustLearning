/*
假设我们在编写一个游戏，玩家可以用代币购买物品。每个物品花费 5 个代币，每次购买还有 1 个代币的手续费。
玩家输入想要购买的数量，total_cost 函数会计算总成本。由于玩家输入的是字符串，他们可能输入任何内容，不一定是数字！
目前，这个函数完全没有处理错误情况。
我们想做的是：如果我们在一个非数字的字符串上调用 total_cost 函数
该函数会返回一个 ParseIntError。在这种情况下，我们希望立即从我们的函数返回该错误，而不尝试进行乘法和加法。
至少有两种实现方式都是正确的。但其中一种要简短得多！
*/

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Handle the error case as described above.
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
