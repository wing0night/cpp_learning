## `for` 循环

所有权转移问题
![alt text](image.png)

效率&安全问题
```rust
// 第一种
let collection = [1, 2, 3, 4, 5];
for i in 0..collection.len() {
  let item = collection[i];
  // ...
}

// 第二种
for item in collection {

}
```
两种循环方式的对比：第二种在访问时是连续的，更安全，不会存在在两次循环之间`collection`发生改变而出现脏数据；第二种在访问时编译器已经证明索引不会超出`collection`长度，而第一种在运行时每次循环都要进行检查，第二种速度也更快

while等循环方式与第一种是同一个道理，所以说`for`是Rust的大杀器








