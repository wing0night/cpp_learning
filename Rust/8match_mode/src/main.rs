#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// 枚举嵌套
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
 }
 
 enum Message {
     Quit,
     Move { x: i32, y: i32 },
     Write(String),
     ChangeColor(Color),
 }

// 模式匹配的另外一个重要功能是从模式中取出绑定的值
// 其中 Coin::Quarter 成员还存放了一个值：美国的某个州
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

// 模式匹配更复杂的元组例子
enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn main() {
    // match
    let dire = Direction::South;
    // match的所有分支的表达式最终返回值的类型必须相同
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => { // 逻辑运算符`或`
            println!("South or North");
        },
        _ => println!("West"), // 用 _ 来代表未列出的所有可能性，类似于default
    };

    let coin = Coin::Penny;
    let value = value_in_cents_plus_state(coin);
    println!("The value of the coin is {}", value);

    let coin_state = Coin::Quarter(UsState::Alaska);
    let value_state = value_in_cents_plus_state(coin_state);
    println!("The value of the coin is {}", value_state);

    // match的分支本身也是表达式，因此也可以利用match进行赋值
    enum IpAddr {
        Ipv4,
        Ipv6
     }
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };
    println!("{}", ip_str);

    // 利用match进行模式匹配同时取出模式中的值更复杂的例子
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1,2),
        Action::ChangeColorRGB(255,255,0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    // 在match中使用_忽略其他情况
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    // 也可以用一个变量来承载其他情况
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };

    // 只需要匹配一个情况时可以使用if let
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }

    // matches! 宏，也是模式匹配的一种形式
    // matches! 宏的第一个参数是要匹配的值，第二个参数是模式
    // 如果第一个参数匹配第二个参数的模式，那么 matches! 宏返回 true，否则返回 false
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    // 变量遮蔽
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(x) =>  println!("匹配出来的age是{}", x),
        // 这里如果使用Some(age) =>  println!("匹配出来的age是{}",age),会发生变量遮蔽
        _ => ()
    }
    println!("在匹配后，age是{:?}", age);

    // 匹配Option的值（是否为None）
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // while let条件循环
    // 只要模式匹配就一直进行 while 循环
    // 以下代码使用while let弹出栈中的每一个元素
    // Vec是动态数组
    let mut stack = Vec::new();
    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for循环 + 模式匹配
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 函数参数的模式匹配
    let point = (3, 5);
    print_coordinates(&point);

    // let和if let
    // let Some(x) = some_option_value; // 这是错误的，不能直接使用let，遗漏了 None 的匹配
    // if let Some(x) = some_option_value { // 这是正确的，if let允许匹配一种模式，而忽略其余的模式
    //     println!("{}", x);
    // }
    
    // if let 和 let else
    // if let
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }
    // let-else
    // let else的语句中x的值可以不限于特定分支使用，可以被传递出去
    // let Some(x) = some_option_value else { return; }
    // println!("{}", x);

    // 在match中通过..=引入闭区间
    let x = 5;
    match x {
        1..=5 => println!("one through five"), // 1~5都会执行该分支
        _ => println!("something else"),
    }

    // 模式匹配例子
    // 利用了上面定义的嵌套枚举
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }

    // _忽略match中模式匹配的值
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }
    // 用.. 忽略剩余的值
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 匹配守卫
    // 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 使用匹配守卫来解决变量覆盖的问题
    let x = Some(5);
    let y = 10;
    // 多加一个if，相当于不用再受限于只有一个条件而无法同时完成值转移和判断
    match x {
        Some(50) => println!("Got 50"),
        // 相比指定会覆盖外部 y 的模式 Some(y)，这里指定为 Some(n)。此新建的变量 n 并没有覆盖任何值，因为 match 外部没有变量 n
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    // 匹配守卫的优先级
    // (4 | 5 | 6) if y => ...正确
    // 4 | 5 | (6 if y) => ...错误
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @绑定
    // 允许为一个字段绑定另外一个变量
    enum Message_hello {
        Hello { id: i32 },
    }
    let msg = Message_hello::Hello { id: 5 };
    match msg {
        // 测试 Message::Hello 的 id 字段是否位于 3..=7 范围内，同时也希望能将其值绑定到 id_variable 变量中以便此分支中相关的代码可以使用它
        Message_hello::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message_hello::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message_hello::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }

    // @支持在绑定的同时解构
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    #[derive(Debug)]
    struct Point_2 {
        x: i32,
        y: i32,
    }
    let p @ Point_2 {x: px, y: py } = Point_2 {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);
    let point = Point_2 {x: 10, y: 5};
    if let p @ Point_2 {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }


}

// 函数参数的模式匹配
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// 使用_忽略函数的参数
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn value_in_cents_plus_state(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 在匹配 Coin::Quarter(state) 模式时，我们把它内部存储的值绑定到了 state 变量上，因此 state 变量就是对应的 UsState 枚举类型。
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}



