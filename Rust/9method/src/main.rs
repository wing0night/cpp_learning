struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
// 使用impl关键字来定义方法
// rust中方法与对象定义是分开的，这里的对象是Circle结构体
impl Circle {
    // new是Circle的关联函数（构造器），因为它的第一个参数不是self，且new并不是关键字
    // 这种方法往往用于初始化当前结构体的实例
    // 实际上不是方法（没有self传入）
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle的方法，&self表示借用当前的Circle结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// 另一个例子
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 定义了一个 Rectangle 结构体，并且在其上定义了一个 area 方法，用于计算该矩形的面积
impl Rectangle {
    // 在 area 的签名中，我们使用 &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写
    // 这里的self也有所有权的概念，self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少；&self 表示该方法对 Rectangle 的不可变借用；&mut self 表示可变借用
    // 因此很少直接使用self，而是使用&self或&mut self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// rust允许多个impl块，比如可以在不同的impl块中实现不同的trait
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // let m = Message::Write(String::from("hello"));
    // m.call();
}

pub struct Rectangle2 {
    width: u32, // 私有字段
    pub height: u32, // 公有字段
}
impl Rectangle2 {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle2 { width, height }
    }
    // 返回宽度的值
    pub fn width(&self) -> u32 {
        return self.width;
    }
    pub fn height(&self) -> u32 {
        return self.height;
    }
}

// 除了结构体，也可以为枚举实现方法
#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}


