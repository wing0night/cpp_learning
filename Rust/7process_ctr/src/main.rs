fn main() {
    // if-else语句
    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // for 循环
    // for ... in
    // 使用 for 时我们往往使用集合的引用形式，因为被引用者所有权会被转移
    // 但是对于实现了 copy 特征的数组不会
    for i in 1..=5 {
        println!("{}", i);
    }

    // 在循环中获取元素的索引
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    // 同时获取索引和元素值
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // 用 _ 来替代 i 用于 for 循环中，在 Rust 中 _ 的含义是忽略该值或者类型的意思
    for _ in 0..10 {
        // ...
      }
    
    // continue跳过本次循环并继续下一次循环
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }
    // break直接跳出整个循环
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("{}", i);
    }

    // while循环写法
    let mut n = 0;
    while n <= 5  {
        println!("{}!", n);

        n = n + 1;
    }
    println!("我出来了！");

    // loop + if + break写法
    let mut n = 0;
    loop {
        if n > 5 {
            break
        }
        println!("{}", n);
        n+=1;
    }
    println!("我出来了！");

}
