fn main(){
    let mut array:[i32; 3]=[0; 3];//默认是0
    array[1] = 1;
    array[2] = 2;
    for x in &array{
        println!("{}", x);
    }
}
