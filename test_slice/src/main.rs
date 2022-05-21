/*
    slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。
*/
fn main() {
    let str01 = String::from("hello world");

    let tag = first_word(&str01);
    println!("{}", tag);
    println!("{}", b'a');
    println!("{}", 98_222);
    println!("{}", 0b00_11);
    println!("{}", 0xA);
    println!("{}", 0o11);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
