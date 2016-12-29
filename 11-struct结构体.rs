fn main(){
    //struct 
    struct Point {
        x:i32,
        y:i32,
    }
    let mut point = Point{ x: 3, y: 3};
    point.x = 1;
    point.y = 2;
    println!("point.x = {} , point.y = {}.", point.x, point.y);


    //tuple struct 
    struct Color(u8,u8,u8);
    let android_green = Color(0xa4,0xc6,0x39);
    let Color(red,green,blue) = android_green;
    println!("red = {}, green = {}, blue = {}.", red, green, blue);

    //newtype: only one element tuple struct 
    struct Char(i32);
    let a = Char(97);
    let Char(int_char_a) = a;
    println!("int_char_a = {}.", int_char_a);

    // unit-like structs 类单元结构体
    // 这样的结构体叫做“类单元”因为它与一个空元组类似，()，这有时叫做“单元”。
    // 就像一个元组结构体，它定义了一个新类型。
    // 就它本身来看没什么用（虽然有时它可以作为一个标记类型），不过在与其它功能的结合中，它可以变得有用。
    // 例如，一个库可能请求你创建一个实现了一个特定特性的结构来处理事件
    // 。如果你并不需要在结构中存储任何数据，你可以仅仅创建一个类单元结构体。
    struct EmptyStruct;
    let empty = EmptyStruct;

    //使用..忽略一些值，并从其他struct中拷贝
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    let defult_point3d = Point3d{x:1, y:1, z:1};
    let point3d = Point3d{x:2, ..defult_point3d};

    //
}