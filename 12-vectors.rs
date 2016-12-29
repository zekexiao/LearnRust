fn main(){
    let a = vec![1,2,3,4,5,6,7,8,9,0];
    println!("a[0] = {}, a[9] = {}", a[0], a[9]);

    let b = vec![0;10];//十个零
    println!("b[0] = {}, b[9] = {}", b[0], b[9]);


    // need use usize to index 
    let c:i32 = 1;
    // let d = b[c];//error
    let d = b[c as usize];
    println!("d = {}", d);


    let e = a;
    // let f = e[10];//index out of bounds , error
    // panic
    match e.get(10) {
        Some(x) => println!("e[10] = {}", x),
        None => println!("index out of bounds"),
    }

    let mut g = vec![1,2,3];
    // immutable reference
    for i in &g {
        print!("{}\t", i);
    }
    println!("");
    // mutable reference 
    for i in &mut g {
        *i += 1;
        print!("{}\t", i);
    }
    //now g == vec![2,3,4];
    println!("");
    // take ownership of the vector and its element 
    for i in g {
        print!("{}\t", i);
    }
    // ownership has been taked , cant use it again
    // for i in g {
    //     print!("{}\t", i);
    // }

    // but u can reference many times

}