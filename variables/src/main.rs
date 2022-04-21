fn main() {
    // 默认定义的变量不能改变值
    let x = 5;
    println!("x = {}", x);
    // 通过mut修饰的变量可以修改值
    let mut y = 4;
    println!("y = {}", y);
    y = 7;
    println!("y = {}", y);
    // 定义常量 声明常量使用 const 关键字而不是 let，并且必须注明值的类型
    const COUNT: i8 = 2;
    println!("COUNT = {}", COUNT);
    // 定义一个与之前变量同名的新变量。则称之为第一个变量被第二个 隐藏 了
    let x = 9;
    println!("x = {}", x);
    // 元组类型
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
    // 元组的第一个索引值是 0
    let tup: (u8, bool, char, f64) = (2, true, '🎃', 1.22);
    // 解构
    let (a, b, c, d) = tup;
    println!("The value of y is: {}", a);
    println!("The value of y is: {}", b);
    println!("The value of y is: {}", c);
    println!("The value of y is: {}", d);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    println!("{}", tup.3);

    // 数组
    // 数组中的每个元素的类型必须相同。数组长度是固定的。
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("1月：{}", _months[0]);
    println!("2月：{}", _months[1]);
    println!("3月：{}", _months[2]);
    println!("4月：{}", _months[3]);
    println!("5月：{}", _months[4]);
    println!("6月：{}", _months[5]);
    println!("7月：{}", _months[6]);
    println!("8月：{}", _months[7]);
    println!("9月：{}", _months[8]);
    println!("10月：{}", _months[9]);
    println!("11月：{}", _months[10]);
    println!("12月：{}", _months[11]);

}
