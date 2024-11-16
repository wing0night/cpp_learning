// 特征trait定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。
// #[derive(Debug)]和std::ops::Add都是trait

// 如果不同的类型具有相同的行为，那么我们就可以定义一个特征，然后为这些类型实现该特征
// 特征声明
// 接下来，每一个实现这个特征的类型都需要具体实现该特征的相应方法
pub trait smrz {
    // 特征只定义行为看起来是什么样的，而不定义行为具体是怎么样的。
    // 因此，我们只定义特征方法的签名，而不进行实现，此时方法签名结尾是 ;，而不是一个 {}。
    fn smrz(&self) -> String;
}

// 为post和weibo实现特征的例子
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

// 实现特征的语法与实现方法很像
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}




