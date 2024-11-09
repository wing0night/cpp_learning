#![allow(unused_variables)] // 允许被定义变量未使用
fn main() {
    cut();
    type_string();
    escape();
    non_escape();
    operate_utf8();
}

// 切片
// 切片即部分引用
fn cut(){
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    let slice = &s[..2]; // 从0开始
    let len = s.len();
    let slice_end = &s[4..len]; // 到结尾
    let zh = "中国人";
    // let a = &zh[0..2]; // 会导致程序崩溃，因为utf-8大部分中文字符的字节长度是3，要取到边界
    // println!("{}",a);

    // 数组的切片
    let a = [1, 2, 3, 4, 5];
    let slice_num = &a[1..3];
    assert_eq!(slice_num, &[2, 3]);
}

// 字符串的插入、替换、删除
fn type_string(){
    let mut s = String::from("Hello ");
    // String类型的追加
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);
    s.push('!');
    println!("追加字符 push() -> {}", s);
    // String类型的插入
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);

    // 替换（与替换相关的方法有3个）
    // replace()方法（返回一个新的字符串，而不是操作原来的字符串）
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
    // replacen()方法，加上一个替换个数参数（也是返回一个新的字符串）
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replacen);
    // replace_range()方法（替换指定范围的字符串）（直接操作原有字符串，不会返回新字符串）
    let mut string_replace_range = String::from("I like rust!"); // mut声明为可变变量
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    // 删除
    // pop()方法（删除最后一个字符，并返回这个字符）
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    // remove()方法（删除指定位置的字符，并返回这个字符）
    // 这个方法是操作索引->如果参数所给的位置不是合法的字符边界，则会发生错误
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字，这行代码不会出错
    // string_remove.remove(3);
    dbg!(string_remove);
    // truncate()方法
    // 删除指定位置到结尾的所有字符
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    // clear()方法
    // 清空字符串
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);

    // 字符串的连接
    // 使用+、+=
    // 使用+连接字符串时，相当于调用了add()方法
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!"; // `+=` 运算符会获取 `result` 的可变引用
    println!("连接字符串 + -> {}", result);
    // 调用add()时所有权转移问题
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // 因为s1相当于被传递给add方法，所有权被转移，而s2的所有权没有被转移
    // println!("{}",s1);
    // 另一种方法是使用format!宏连接字符串
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}

// 字符串转义
fn escape(){
    // 使用\输出ASCII和unicode
    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}

// 希望本身出现转义字符的字符串不要转义，保持原样
fn non_escape(){
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中包含 # 号，可以在开头和结尾加多个 # 号，最多加255个，只需保证与字符串中连续 # 号的个数不超过开头和结尾的 # 号的个数即可
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

// 操作utf_8字符串
fn operate_utf8(){
    // 使用chars()方法
    for c in "中国人".chars() {
        println!("{}", c);
    }
    // 用bytes返回字符串底层字节数组
    for b in "中国人".bytes() {
        println!("{}", b);
    }
    
}






