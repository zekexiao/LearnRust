fn main(){
    loop{
        println!("Hello!Loop");
        break;//退出循环
    }//无限循环
    let mut x = 10;
    while !(x==0){
        println!("{}",x );
        x = x - 1;
    }//while循环，条件为true时执行
    for j in 0..10{
        println!("{}",j );
    }//感觉就像C#里面的foreach一样，迭代器
    for (a,b) in (0..10).enumerate() {
        println!("a = {} , b = {}",a,b );
    }//这玩意真厉害，a代表循环的次数，b是迭代器里的每个元素
    loop{
        println!("Hello!Loop");
        break;//退出循环
    }
    while x==0{
        println!("x = {}",x );
        continue;//退出本次循环
        x = x + 1;//这句将不会执行
    }//这是一个无限循环

}
