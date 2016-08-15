fn main(){
    let x = 5;
    //if
    if x == 5{
        println!("x is five");
    }
    //if-else
    if x == 6{
        println!("x is six");
    }else{
        println!("x is not six");
    }
    //if-else if
    if x == 7{
        println!("x is seven");
    }else if x == 8{
        println!("x is enght");
    }else{
        println!("hahahallalala");
    }
    //we can use if to bind mut&immut
    let a = if x== 5 { 6 } else { 7 };
    println!("{}",a );
}
