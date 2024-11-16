## 有关泛型

在 Rust 中泛型是**零成本**的抽象，意味着你在使用泛型时，完全不用担心性能上的问题。

但是任何选择都是权衡得失的，既然我们获得了性能上的巨大优势，那么又失去了什么呢？Rust 是在编译期为泛型对应的多个类型，生成各自的代码，因此**损失了编译速度**和增大了最终生成文件的大小。



## 枚举中的泛型

枚举用于函数返回值。

Result 关注的主要是值的正确性。

如果函数正常运行，则最后返回一个 Ok(T)，T 是函数具体的返回值类型，如果函数异常运行，则返回一个 `Err(E)`，E 是错误类型。例如打开一个文件：如果成功打开文件，则返回 `Ok(std::fs::File)`
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## const fn 例子


```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const RESULT: usize = add(5, 10);

fn main() {
    println!("The result is: {}", RESULT);
}
```









