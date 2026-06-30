//这是针对以下部分的测验：
//-变量
//-函数
//-如果
//
//玛丽正在买苹果。一个苹果的价格计算如下：
//-一个苹果需要 2 个锈币。
//-但是，如果玛丽购买了超过 40 个苹果，则每个苹果的价格
//整个订单减少到只有 1 个 rustbuck！

//TODO: 编写一个函数来计算给定苹果订单的价格
//购买的数量。
fn calculate_price_of_apples(count: i32) -> i32 {
    if count <= 40 { count * 2 } else { count }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
