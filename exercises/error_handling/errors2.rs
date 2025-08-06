// errors2.rs
//
// 假设我们正在编写一款游戏，在游戏中你可以用代币购买物品。所有物品每件售价5个代币，而且每次购买物品都会收取1个代币的手续费。
// 游戏玩家会输入他们想要购买的物品数量，total_cost 函数将计算所需代币的总成本。
// 不过，由于玩家输入的数量是以字符串形式获取的，而且他们可能输入任何内容，并非仅仅是数字！
//
// 目前，这个函数完全没有处理错误情况（并且也没有正确处理成功情况）。我们要做的是：
// 如果对一个非数字的字符串调用 parse 函数，该函数将返回一个 ParseIntError，在这种情况下，我们希望立即从我们的函数中返回这个错误，而不是尝试进行乘法和加法运算。
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.


use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    // let qty = item_quantity.parse::<i32>()?;
    // Ok(qty * cost_per_item + processing_fee)
    let qty = item_quantity.parse::<i32>();
    match qty{
        Ok(val) => Ok(val * cost_per_item + processing_fee),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
