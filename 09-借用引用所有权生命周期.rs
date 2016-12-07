// 所有权：http://rustbook.cn/content/Ownership%20所有权.html
// 引用和借用：http://rustbook.cn/content/References%20and%20Borrowing%20引用和借用.html
// 生命周期：http://rustbook.cn/content/Lifetimes%20生命周期.html
// 这3章依次互相关联所以同一到一个文件
//第一，任何借用必须位于比拥有者更小的作用域。第二，同一个作用域下，不能同时存在可变和不可变的引用
fn main(){//所有权：Rust 中的变量绑定有一个属性：它们有它们所绑定的的值的所有权。这意味着当一个绑定离开作用域，它们绑定的资源就会被释放。
    let v = vec![1,2,3];
    let v1 = v;
    // println!("{}",v[0]);
    // 编译器会提示vec!<i32>没有实现copy trait
    // 第一行为 vector 对象v和它包含的数据分配了内存。向量对象储存在栈上并包含一个指向堆上[1, 2, 3]内容的指针。
    // 当我们从v移动到v2，它为v2创建了一个那个指针的拷贝。这意味着这将会有两个指向向量内容的指针。
    // 这将会因为引入了一个数据竞争而违反 Rust 的安全保证。因此，Rust 禁止我们在移动后使用v

    let i = 1;
    let i2 = i;
    println!("i = {}, i2 = {}",i ,i2 );
    //而i32类型，实现了copy trait,所以这一行并不会报错
    let sum = sum(&i,&i2);
    println!("{}", sum);
    foo(&v1);
    //可变量引用
    let mut x = 0;
    {
        let y = &mut x;
        *y += 1;
    }

    println!("{}",x );//x = 1;
    //引用必须与它引用的值存活得一样长。当引用在它引用的变量之前声明会导致类似的问题
    {
        let y:&i32;
        let x = 10;
        y = &x;//Error
        //y在x之前声明，比x的存活时间更长
    }
    {
        let y:&i32;
        {
            let x = 10;
            y = &x;
        }//Error
    }
}
fn sum(v1: &i32, v2: &i32) -> i32{//引用和借用，使用此方法可以直接借用变量，使得变量在作用域结束后不会被释放
    v1 + v2
}
fn foo(v: &Vec<i32>){//引用是不可变的，就像绑定一样。这意味着在foo()中，向量完全不能被改变
    v.push(6);//Error
}
