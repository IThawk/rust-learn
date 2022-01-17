fn main() {
    println!("Hello, world!");
    println!("12+13={}", add(12, 13));
    let user1 = User {
        name: "LISI".to_string(),
        age: 12,
    };
    println!("name:{},age:{}", user1.name, user1.age);
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug, Clone)]
struct User {
    name: String,
    age: i32,
}
#[derive(Debug)]
enum Language {
    ZHR = 1,
    ENG = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);
#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}
