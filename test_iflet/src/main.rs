fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    /*
    if let 语法获取通过等号分隔的一个模式和一个表达式。它的工作方式与 match 相同，这里的表达式对应 match 而模式则对应第一个分支。
    在这个例子中，模式是 Some(max)，max 绑定为 Some 中的值。
    接着可以在 if let 代码块中使用 max 了，就跟在对应的 match 分支中一样。
    模式不匹配时 if let 块中的代码不会执行。

    换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
    可以在 if let 中包含一个 else。else 块中的代码与 match 表达式中的 _ 分支块中的代码相同，这样的 match 表达式就等同于 if let 和 else。
    */
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
