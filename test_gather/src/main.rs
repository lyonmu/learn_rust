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
    2.字符串
        - 创建字符串变量的方式: String::new() "initial contents".to_string()  String::from("wdwd")
        - 字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
        - 使用 push_str 和 push 附加字符串
        - 使用 + 运算符或 format! 宏拼接字符串
        - Rust 的字符串不支持索引。
        - 操作字符串每一部分的最好的方法是明确表示需要字符还是字节。
    3.HashMap
        - 可以使用 new 创建一个空的 HashMap，并使用 insert 增加元素。
        - 哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
        - 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。
        - 可以通过 get 方法并提供对应的键来从哈希 map 中获取值
        - 如果我们插入了一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换。
        - entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。
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

    let mut s = String::from("wdwd");
    s.push('a');
    println!("{}", s);

    let data = "initial contents";

    let s1 = data.to_string();
    println!("{}", s1);

    // 该方法也可直接用于字符串字面值：
    let s2 = "initial contents".to_string();
    println!("{}", s2);

    let s3 = format!("{}-{}-{}", s1, s2, s);
    println!("{}", s3);

    let s4 = s1 + &s2;
    println!("{}", s4);

    let hello = String::from("Hola");
    println!("{}", hello.len());

    let hello01 = String::from("Здравствуйте");
    println!("{}", hello01.len());

    // 输出字符
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // 输出字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores01: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    scores01.entry(String::from("Red")).or_insert(30);

    println!("{:?}", scores01.get(&String::from("Blue")));
    println!("{:?}", scores01);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
