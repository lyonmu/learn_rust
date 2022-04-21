fn main() {
    println!("Hello, world!");
    other_function();
    sum(259, 111);
    let sum = sum2(123, 0);
    let result = if sum > 123 {
        println!("sum > {}", sum)
    } else if sum < 123 {
        println!("sum < {}", sum)
    } else {
        println!("sum = {}", sum)
    };
    println!("result = {:?}", result);
    let mut count = 0;
    let test = loop {
        println!("{}", count);
        count += 1;
        if count == 10 {
            break count * 20;
        }
    };
    println!("{:?}", test);

    while count != 0 {
        println!("开始减一：{}", count);
        count -= 1;
    }
    println!("遍历元组");
    let tup: (i32, bool, f64, char, &str) = (1212, true, 12.32, 'x', "uqingu");
    let arrays = [1, 2, 3, 4, 5];
    let mut number = 0;

    while number != 5 {
        println!("{:?}", tup);
        println!("{:?}", arrays[number]);
        number += 1;
    }

    for e in arrays {
        println!("e = {}", e);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
fn other_function() {
    println!("test function!!!")
}
fn sum(x: u32, y: u32) {
    println!("x + y = {}", x + y);
}
fn sum2(x: u32, y: u32) -> u32 {
    x + y
}
