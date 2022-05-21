/*
    定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。
    结构体的名字需要描述它所组合的数据的意义。
    接着，在大括号中，定义每一部分数据的名字和类型，我们称为 字段（field）。

    方法:
        为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写），这个 impl 块中的所有内容都将与 Rectangle 类型相关联。
    接着将 area 函数移动到 impl 大括号中，并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数改成 self。
    然后在 main 中将我们先前调用 area 方法并传递 rect1 作为参数的地方，改成使用 方法语法（method syntax）在 Rectangle 实例上调用 area 方法。
    方法语法获取一个实例并加上一个点号，后跟方法名、圆括号以及任何参数。
        在 area 的签名中，使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写。
    在一个 impl 块中，Self 类型是 impl 块的类型的别名。
    方法的第一个参数必须有一个名为 self 的Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写。
    注意，我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例，就像我们在 rectangle: &Rectangle 中做的那样。
    方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。

    关联函数:
    所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关。
    我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。
    我们已经使用了一个这样的函数：在 String 类型上定义的 String::from 函数。

    每个结构体都允许拥有多个 impl 块。
*/

// 使用没有命名字段的元组结构体来创建不同的类型
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 基本结构体
struct User {
    id: usize,
    name: String,
    email: String,
    socre: f64,
}
// 没有任何字段的类单元结构体
// struct AlwaysEqual;

// 不在方法块中的构造函数
fn build_user(email: String, name: String, id: usize, socre: f64) -> User {
    User {
        email,
        name,
        id,
        socre,
    }
}
// 添加 #[derive(Debug)] 标识实现调试打印结构体实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 方法
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 使用关联函数创建构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /*
    一旦定义了结构体后，为了使用它，通过为每个字段指定具体值来创建这个结构体的 实例。
    创建一个实例需要以结构体的名字开头，接着在大括号中使用 key: value 键-值对的形式提供字段，其中 key 是字段的名字，value 是需要存储在字段中的数据值。
    实例中字段的顺序不需要和它们在结构体中声明的顺序一致。
    使用 mut 关键字可以修改构建的实例
    */
    let mut user = User {
        id: 1213455,
        name: "someusername123".to_string(),
        email: "uqingu@foxmail.com".to_string(),
        socre: 98.3,
    };
    // 为了从结构体中获取某个特定的值，可以使用点号。
    user.name = "muqing".to_string();

    println!("{}", user.id);
    println!("{}", user.name);
    println!("{}", user.email);
    println!("{}", user.socre);

    let user01 = build_user(
        "muqing@foxmail.com".to_string(),
        "mumu".to_string(),
        12398,
        89.3,
    );

    println!("{}", user01.id);
    println!("{}", user01.name);
    println!("{}", user01.email);
    println!("{}", user01.socre);

    // 使用结构体更新语法从其他实例创建实例
    let user02 = User { id: 121312, ..user };

    println!("{}", user02.id);
    println!("{}", user02.name);
    println!("{}", user02.email);
    println!("{}", user02.socre);

    // 注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型。
    let black = Color(0, 0, 0);
    println!("{}{}{}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("{}{}{}", origin.0, origin.1, origin.2);

    // let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", rect1.width);
    println!("{}", rect1.height);
    println!("rect1 is {:#?}", rect1);
    // dbg! 宏接收一个表达式的所有权，打印出代码中调用 dbg! 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(48);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area()
    );
}
