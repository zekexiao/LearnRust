fn main(){
    let day = 5;
    match day {
        6 | 7 | 8 => println!("weekend"),
        1...5 => println!("weekday"),
        _ => println!("FK"),
    }
    //使用','分开每句
    // "|"用于匹配多个值,如： 5|4 匹配5和4.
    // "..."用于匹配一个范围,如：1...5 匹配1,2,3,4,5.
    // "_"用于匹配其他值，因为match必须覆盖所有可能值.

    let x = 1;
    match x {
        e @ 1...5 => println!("x = {}", e),
        f @ 7 |f @ 8 | f @ 9 => println!("x = {}", f),
        _ => println!("Balabala"),
    }
    //使用@获得|或者...匹配到的值.

    let y = 5;
    let mut z = 5;
    match y {
        ref r => println!("r = {}", r),
    }
    match z {
        ref mut mutr => {
            *mutr = 10;
            println!("z = {}", mutr);
        },
    }
    //用ref获得一个引用，可以是mut可以是immut

    let a = (0, 2);
    match a {
        //(0, y) => println!("x = 0, y = {}", y),
        (x, 0) => println!("x = {}, y = 0", x),
        (0, 2) => println!("x = 0, y = 2"),
        _ => println!("Balabala"),
    }
    //可以用match匹配元组

    //wait more...
}
