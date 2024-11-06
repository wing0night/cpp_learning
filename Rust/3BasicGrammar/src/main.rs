// Rust 程序入口函数，跟其它语言一样，都是 main，该函数目前无返回值
fn main() {
    
    state_var();
    learn_type();
    math_operation();
    place_operation();
}

// 变量声明
fn state_var(){
    // 使用let来声明变量，进行绑定，a是不可变的
    // 此处没有指定a的类型，编译器会默认根据a的值为a推断类型：i32，有符号32位整数
    // 语句的末尾必须以分号结尾
    // = 等号接数值，：冒号接类型
    let a = 10;
    // 主动指定b的类型为i32
    let b: i32 = 20;
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let mut c = 30i32;
    // 还能在数值和类型中间添加一个下划线，让可读性更好
    let d = 30_i32;
    // 跟其它语言一样，可以使用一个函数的返回值来作为另一个函数的参数
    let e = add(add(a, b), add(c, d));

    let _x = 5; //使用下划线告知，这个变量不会被使用，避免编译器警告

    //常量（约定大写）
    const MAX_POINTS: u32 = 100_000;

    // println!是宏调用，看起来像是函数但是它返回的是宏定义的代码块
    // 该函数将指定的格式化字符串输出到标准输出中(控制台)
    // {}是占位符，在具体执行过程中，会把e的值代入进来
    println!("( a + b ) + ( c + d ) = {}", e);
    println!("MAX_POINTS = {}", MAX_POINTS);

    c = 40;
    println!("{}", c);

    // 变量屏蔽（允许用不同类型的变量覆盖）
    // 而let mut 是不允许用不同类型覆盖的
    // 字符串类型
    let spaces = "   ";
    // usize数值类型
    let spaces = spaces.len(); // 这句话中也借用了上一个spaces的值
    println!("spaces = {}", spaces);
}

// 定义一个函数，输入两个i32类型的32位有符号整数，返回它们的和
fn add(i: i32, j: i32) -> i32 {
    // 返回相加值，这里可以省略return
    i + j
}

fn learn_type(){
    // 整数类型
    let _a:i32=30; // 有符号。i32为默认类型
    let _b:u32=20; // 无符号
    // 整数溢出（可以通过函数显式解决）

    //浮点数
    let _d:f64=3.1415926; // 双精度浮点数。f64为默认类型
    let _c:f32=3.14; // 单精度浮点数
    // 浮点数陷阱
    let _e:f64=0.1+0.2; // 0.30000000000000004
    // assert!(0.1 + 0.2 == 0.3); // 这个断言会失败
    // 以下是关于单精度和双精度浮点数的一个例子
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();
    // \n
    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();
    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2); // 这句断言会失败，因为双精度比较时会有精度问题

    // 负数开方，返回NaN
    let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x); // 会导致程序崩溃，NAN不能用来比较
    println!("x = {}", x);
}

fn math_operation(){
    // 数学运算
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;
    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
}

// 位运算
fn place_operation(){
    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;
    // 二进制为00000011
    let b: u8 = 3;

    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);
    println!("b value is        {:08b}", b);
    println!("(a & b) value is  {:08b}", a & b); // 与运算00000010 & 00000011 = 00000010
    println!("(a | b) value is  {:08b}", a | b); // 或运算00000010 | 00000011 = 00000011
    println!("(a ^ b) value is  {:08b}", a ^ b); // 异或运算00000010 ^ 00000011 = 00000001
    println!("(!b) value is     {:08b}", !b); // 非运算00000011 -> 11111100
    println!("(a << b) value is {:08b}", a << b); // 左移运算00000010 << 00000011 = 00001000
    println!("(a >> b) value is {:08b}", a >> b); // 右移运算00000010 >> 00000011 = 00000000

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {:08b}", a);
}


