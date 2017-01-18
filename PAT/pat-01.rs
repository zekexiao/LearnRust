use std::io;
fn main(){ 
    let mut input = String::new();
    io::stdin().read_line(&mut input)
            .expect("Failed read line");
    let mut num:i32 = input.trim()
                .parse()
                .expect("Falied convert to i32");
    let mut times = 0;
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        }else{
            num = (3*num + 1) / 2;
        }
        times += 1;
    }
    println!("{}", times);
}