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



