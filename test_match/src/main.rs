/*
    match:
        1.语法:
            match 参与表达式运算的变量 {
                表达式 1 => { 符合表达式后的逻辑},
                表达式 2 => { 符合表达式后的逻辑},
                表达式 3 => { 符合表达式后的逻辑},
                ...
            }
        2.Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。
            这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。
        3.通配模式允许我们通过命名一个变量来创建一个通配分支,该分支将匹配所有未被特殊列出的值。
            这种通配模式满足了 match 必须被穷尽的要求。
            必须将通配分支放在最后，因为模式是按顺序匹配的。
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is Penny {}", 1);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let num = Coin::Penny;
    value_in_cents(num);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);
    println!("{:?}", none);
}
