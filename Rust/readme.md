## command

`rustup doc`本地帮助文档

## cargo

Rust 包管理器

### 创建&编译项目
`cargo new helloworld`（创建一个可运行的项目）

创建依赖库项目`cargo new helloworld --bin`

`cd hellloworld`

`cargo run` 编译+运行项目

`cargo build` 编译。默认为debug模式（即开发模式，注重debug速度而非代码运行速度）

`./target/debug/helloworld` 运行

`cargo build --release` release模式编译，代码运行速度快

`cargo check` 快速检查代码能否编译通过，可以节省大量编译时间

### 描述文件

`cargo.toml` 项目数据描述文件，合理构建使得开发者能够根据自己期望的方式对项目进行构建、测试、运行

`cargo.lock` 项目依赖清单





