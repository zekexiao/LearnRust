use std::io;

fn main(){
    let mut sum = 0;
    let mut input = String::new();
    io::stdin().read_line(&mut input)
                .expect("Failed");
    for i in input.trim().chars() {
        sum += i as i32 - '0' as i32;
    }
    // let a:i32 = input
    //           .trim()
    //           .parse()
    //           .expect("Failed");
    if sum >= 100 {
        println!("{} {} {}", read_number(sum/100), read_number(sum/10%10), read_number(sum%10));
    }else if sum >= 10 {
        println!("{} {}", read_number(sum/10), read_number(sum%10));
    }else {
        println!("{}", read_number(sum));
    }
}
fn read_number(n:i32 ) -> String {
    match n {
        1 => "yi".to_string(),
        2 => "er".to_string(),
        3 => "san".to_string(),
        4 => "si".to_string(),
        5 => "wu".to_string(),
        6 => "liu".to_string(),
        7 => "qi".to_string(),
        8 => "ba".to_string(),
        9 => "jiu".to_string(),
        0 => "ling".to_string(),
        _ => "Failed".to_string(),
    }
}