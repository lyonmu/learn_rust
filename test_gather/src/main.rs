fn main() {
    /*
    1.vector
        - vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。
        - vector 只能储存相同类型的值。
        - 使用 push 可以向 vector 添加新的值
        - Rust 提供了 vec! 宏，这个宏会根据我们提供的值来创建一个新的 vector。
        - 如果想要能够改变 vector 类型变量的值，必须使用 mut 关键字使其可变。
        - for...in... 遍历 vector
        - vector 只能储存相同类型的值。这是很不方便的；绝对会有需要储存一系列不同类型的值的用例。
          幸运的是，枚举的成员都被定义为相同的枚举类型，所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    2.
    */
    let mut v: Vec<i32> = Vec::new();

    v.push(23);
    v.push(24);
    v.push(25);
    v.push(26);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v2.push(5);

    let var = &v[2];
    println!("{}", var);

    if let Some(var2) = v.get(3) {
        println!("{}", var2);
    }

    for i in &v {
        println!("{}", i);
    }

    for element in v.iter() {
        println!("{}", &element);
    }
    
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for element in row.iter() {
        println!("{:?}", &element);
    }
}
