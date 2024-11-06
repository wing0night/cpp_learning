
fn greet(){
    let greet_en = "Hello, world!";
    let greet_cn = "你好，世界！";
    let greet_all = [greet_en, greet_cn];
    for greets in greet_all {
        // 对于 println 来说，我们没有使用其它语言惯用的 %s、%d 来做输出占位符，而是使用 {}，因为 Rust 在底层帮我们做了大量工作，会自动识别输出数据的类型，例如当前例子，会识别为 String 类型。
        println!("{}", &greets);
    }
}

fn main() {
    greet();
}
