/*
    枚举:
        1.通过 enum 关键字定义枚举
        2.枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。
        3.直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
            这里也很容易看出枚举工作的另一个细节：每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。
        4.用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。
        5.可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。甚至可以包含另一个枚举！
        6.结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。
*/

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr01 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Ip {
    kind: IpAddrKind,
    address: String,
}
impl IpAddr {
    fn ip(kind: IpAddrKind, address: String) -> Ip {
        Ip { kind, address }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::ip(IpAddrKind::V4, String::from("127.0.0.1"));
    println!("home = {:#?}", home);

    let loopback = IpAddr::ip(IpAddrKind::V6, String::from("::1"));
    println!("loopback = {:#?}", loopback);

    let home01 = IpAddr::V4(String::from("127.0.0.1"));
    println!("home01 = {:?}", home01);
    let loopback01 = IpAddr::V6(String::from("::1"));
    println!("loopback01 = {:?}", loopback01);

    let home01 = IpAddr01::V4(127, 0, 0, 1);
    println!("home01 = {:?}", home01);
    let loopback01 = IpAddr01::V6(String::from("::1"));
    println!("loopback01 = {:?}", loopback01);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("absent_number = {:?}", absent_number);
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind = {:#?}", ip_kind);
}
