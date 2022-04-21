use std::io;

fn main() {
    println!("输入生成的斐波那契数列长度：");
    let mut fiblen = String::new();
    let mut tag = String::new();
    let mut corf = String::new();
    io::stdin().read_line(&mut fiblen).expect("获取输入失败");
    let fiblen: i32 = fiblen.trim().parse().expect("输入错误！！！");
    fib(fiblen);
    println!("华氏温度输入1，摄氏度输入0：");
    io::stdin().read_line(&mut tag).expect("获取输入失败");
    let tag: i32 = tag.trim().parse().expect("输入错误！！！");
    println!("输入温度：");
    io::stdin().read_line(&mut corf).expect("获取输入失败");
    let corf: f64 = corf.trim().parse().expect("输入错误！！！");
    let c = ftoc(corf, tag);
    println!("温度为：{}", c);
    table();
}

// 编写函数实现斐波那契数列 1 1 2 3 5 8 13 21 34 55 89
fn fib(_n: i32) {
    let mut a = 0;
    let mut b = 1;
    let mut tag;
    let mut index = 0;

    if _n == 1 {
        print!("{}\t", 1);
    } else {
        print!("{}\t", 1);
        while index < _n - 1 {
            tag = a + b;
            a = b;
            b = tag;
            print!("{}\t", tag);
            index += 1;
        }
        println!();
    }
}

// 华氏温度摄氏度互转
fn ftoc(_w: f64, _tag: i32) -> f64 {
    if _tag > 0 {
        println!("华氏温度转摄氏度：");
        (_w - 32.0) / 1.80
    } else if _tag == 0 {
        println!("摄氏度转华氏温度：");
        (_w * 1.80) + 32.0
    } else {
        0.0
    }
}

// 打印九九乘法表
fn table() {
    let mut x = 0;
    let mut y = 0;
    while x < 10 {
        while y < 10 {
            print!("{} X {} = {}\t", y, x, x * y);
            y += 1;
        }
        println!();
        x += 1;
        y = x;
    }
    x = 0;
    y = 0;
    while x < 10 {
        while y <= x {
            print!("{} X {} = {}\t", y, x, x * y);
            y += 1;
        }
        y = 0;
        println!();
        x += 1;
    }
}