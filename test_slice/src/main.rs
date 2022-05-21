/*
    slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。
    字符串 slice（string slice）是 String 中一部分值的引用
        1、对于 Rust 的 .. range 语法，如果想要从索引 0 开始，可以不写两个点号之前的值。
        2、如果 slice 包含 String 的最后一个字节，也可以舍弃尾部的数字。
        3、也可以同时舍弃这两个值来获取整个字符串的 slice。
*/
fn main() {
    let str01 = String::from("hello world");

    let tag = first_word(&str01);
    println!("{}", tag);
    let tag01 = second_word(&str01);
    println!("{}", tag01);

    let hello = &str01[0..5];
    let world = &str01[6..11];
    println!("{}", hello);
    println!("{}", world);

    // 数组也支持 Slice 类型
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    for num in slice.iter() {
        println!("{}", num);
    }
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
