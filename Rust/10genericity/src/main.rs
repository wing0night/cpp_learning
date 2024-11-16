fn main() {
    // 泛型函数参数为T类型，返回值为T类型
    // 但是不是所有类型都能相加，所以需要限定T类型
    // 限定T类型为实现了std::ops::Add的类型
    fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
        a + b
    }

    // 有时候编译器无法推断出类型，需要手动指定类型
    use std::fmt::Display;
    fn create_and_print<T>() where T: From<i32> + Display {
        let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
        println!("a is: {}", a);
    }
    // 在以下代码中手动指定
    create_and_print::<i64>();

    // 在结构体中使用泛型
    // 注意x和y的类型必须相同
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // 制定不同参数，T和U，x和y的类型可以不同
    struct Point2<T,U> {
        x: T,
        y: U,
    }
    let mix = Point2 {x: 5, y: 4.0};

    // 在方法中可以利用泛型为特定的类型实现特定方法
    // 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // const泛型
    // 针对值（而非类型）的泛型
    // 这里的const N用来被标记数组长度，并不代表一种类型
    // 不同长度的array是不同类型
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr);
    }
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);

    // const fn
    // 常量函数，在编译期执行这些函数，从而将计算结果直接嵌入到生成的代码中
    
    

}








