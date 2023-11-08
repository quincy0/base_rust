fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello2 = &s[..5];
    let world = &s[6..11];
    let world2 = &s[6..];

    let len = s.len();
    let hello_world = &s[0..len];
    let hello_world2 = &s[..];

    let s = "中国人";
    let a = &s[0..2];
    println!("{}", a);

    println!("Hello, world!");
}

fn slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello2 = &s[..5];
    let world = &s[6..11];
    let world2 = &s[6..];

    let len = s.len();
    let hello_world = &s[0..len];
    let hello_world2 = &s[..];

    // 注意：切片的索引必须落在字符之间的边界位置。比如UTF-8字符的边界，中文占用三个字节，下面代码就会crash
    let s = "中国人";
    let a = &s[0..2];
    println!("{}", a);
    println!("test");

    let ming = String::from("ming");
    say_hello(&ming);
    say_hello(&ming[..]);
    say_hello(ming.as_str());

    // rust 不允许去索引字符串
    // ming[0]

}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn operate_str() {
    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s); // Hello rust

    s.push('!');
    println!("追加字符 push() -> {}", s); // Hello rust!

    s.insert(5, ',');
    println!("插入字符 insert_str() -> {}", s); // Hello, rust!

    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s); // Hello, I like rust!


}