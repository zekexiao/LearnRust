struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32{//method //方法
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle{//associated functions //关联函数
        Rectangle{ length: size, width: size};
    }
}

fn main(){
    let rect1 = Rectangle{ length: 10, width: 20};
    let rect2 = Rectangle{ length: 20, width: 30};
    let rect3 = Rectangle::square(4);//use associated functions to build a 4X4 square
    
    println!("The area of the rect1 is {} square pixels.",
            rect1.area() );    
    println!("The rect1 can hold rect2?: {}", rect1.can_hold(&rect2));
    println!("The rect2 can hold rect1?: {}", rect2.can_hold(&rect1));
}