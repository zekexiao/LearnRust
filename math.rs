//计划用rust实现我大一C语言作业...
fn main(){

    let a = leijia(100);
    let b = add(20,10);
    let c = jiecheng(5);
    let d = fandu(312);
    let e = gongyueshu(15,7);
    let f = gongbeishu(11,4);
    let g = weishu(-123456789);
    let h = max(10,20);
    let i = min(10,20);
    let j = cifang(2,10);
    let k = is_sushu(11);
    let l = is_pingfangshu(25);
    let m = is_tonggoushu(25);
    let n = is_wanshu(6);
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);
    println!("{}",e);
    println!("{}",f);
    println!("{}",g);
    println!("{}",h);
    println!("{}",i);
    println!("{}",j);
    println!("{}",k);
    println!("{}",l);
    println!("{}",m);
    println!("{}",n);
}

//此函数用来返回a和b的合
fn add(a:i32, b:i32) -> i32 {
    a + b
}
//此函数用来返回1+2+3+…+n，若n<=0,函数用来返回0。
fn leijia(mut a:i32) -> i32 {
    let mut answer=0;
    while a>0 {
        answer = answer + a;
        a = a - 1;
    }
    answer
}

//此函数用来返回n!,若n<=0,函数用来返回0。
//u64可以算到20!
fn jiecheng(n:i32) -> u64 {
    let mut answer:u64 = 1;
    if n <= 0 {
        return 0u64;
    }else{
        for i in 1..n+1 {
            answer = answer * i as u64;
        }
    }
    answer
}

//此函数用来返回n的反读数
fn fandu(mut n:i32) -> i32 {
    let mut answer = 0;
    while n > 0{
        answer = answer * 10 + n%10;
        n = n/10;
    }
    answer
}

//此函数用来计算两个数的最大公约数
//如果两个数小于1或者没有最大公约数（即公约数为1）则返回0
//暴力法
fn gongyueshu(a:i32, b:i32) -> i32 {
    let mut answer = if a > b { b }else{ a };
    if a <= 1 && b <= 1 {
        return 0;
    }else{
        while answer > 1{
            if a%answer == 0 && b%answer == 0{
                return answer;
            }
            answer = answer - 1;
        }
    }
    0
}

//此函数用来求两个数的最小公倍数
//暴力法
fn gongbeishu(a:i32, b:i32) -> i32 {
    let mut answer = if a > b { a }else{ b };
    if a <= 0 && b <= 0 {
        return 0;
    }else{
        while answer < a*b{
            if answer%a == 0 && answer%b == 0{
                return answer;
            }
            answer = answer + 1;
        }
    }
    a*b
}

//此函数返回一个数的位数
fn weishu(mut n:i32) -> i32 {
    let mut answer = 0;
    while n != 0{
        answer = answer + 1;
        n = n/10;
    }
    answer
}

//此函数用来返回两个数中最大的数
fn max(a:i32, b:i32) -> i32 {
    if a > b {
        a
    }else{
        b
    }
}

//此函数用来返回两个数中最小的数
fn min(a:i32, b:i32) -> i32 {
    if a > b {
        b
    }else{
        a
    }
}

//此函数用来返回x的N次方
fn cifang(x:i32, mut n:i32) -> i32 {
    let  mut answer = 1;
    while n!=0{
        answer = answer * x;
        n = n - 1;
    }
    answer
}

//此函数用来判断n是否素数
//若n是素数，则函数返回true，若n不是素数，则函数返回false。
fn is_sushu(n:i32) -> bool {
    for i in 2..n {
        if n%i == 0 {
            return false;
        }
    }
    true
}

//此函数用来判断n是否平方数
//若n是平方数，则函数返回true，若n不是平方数，则函数返回false。
//一个正整数是另外一个正整数的平方，这个数就称为“平方数”
//例如，25=5^2，25就是平方数。(1不是平方数)
fn is_pingfangshu(n:i32) -> bool {
    if n <= 1 {
         return false;
    }else{
        for i in 1..n {
            if i*i == n {
                 return true;
            }
        }
    }
    false
}


//此函数用来判断n是否同构数
//若n是同构数，则函数返回true，若n不是同构数，则函数返回false。
//所谓“同构数”是指这样一个数，它出现在它的平方数的右侧
//例如5的平方是25，25的平方是625，故5和25都是同构数。1不是同构数
fn is_tonggoushu(n:i32) -> bool {
    let mut temp_n = n;
    if n <= 1 {
        return false;
    }
    let mut pow = 1; //存储10的n的位数次方
    while temp_n != 0 {
        temp_n = temp_n / 10;
        pow = pow * 10;
    }
    if n == n*n%pow {
        true
    }else{
        false
    }
}


//此函数用来判断n是否完数
//若n是完数，则函数返回true，若n不是完数，则函数返回false。
//一个数如果恰好等于它的所有真因子之和，这个数就称为“完数”。
//例如，6的真因子为1，2，3，而6=1+2+3，因此，6是“完数”
fn is_wanshu(n:i32) -> bool {
    let mut sum = 0;
    for i in 1..n {
        if n%i == 0 {
            sum = sum + i;
        }
    }
    if sum == n {
        true
    }else{
        false
    }
}
