//计划用rust实现我大一C语言作业...
fn main(){

    let a = leijia(100);
    let b = add(20,10);
    println!("{} , {}",a,b);
}

fn add(a:i32, b:i32) -> i32{
    a + b
}
//此函数用来返回1+2+3+…+n，若n<=0,函数用来返回0。
fn leijia(mut a:i32) -> i32{
    let mut answer=0;
    while a>0 {
        answer = answer + a;
        a = a - 1;
    }
    answer
}

//此函数用来返回n!,若n<=0,函数用来返回0。
fn jiecheng(mut n:i32) -> i64{
    let mut answer:i64 = 1;
    if n <= 0 {
        return 0i64;
    }else{
        for i in 1..n+1 {
            answer = answer * i;
        }
    }
    answer
}
