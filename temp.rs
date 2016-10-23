fn main(){
    let mut a = 1;
    addAandB(&mut a);
    println!("{}", a);
}
fn addAandB(a:&mut i32) -> i32{
      *a += 1
}
