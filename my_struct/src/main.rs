struct User {
    username: String,
    active: bool,
    sign_in_count: u64,
    email: String,
}

fn main() {
    let user1 = User {
        username: String::from("hehe_2324"),
        active: true,
        sign_in_count: 114514,
        email: String::from("hehe@qq.com"),
    };

    let user2 = User {
        username: String::from("xixi"),
        ..user1
    };
    println!("Hello, world from {}", user1.username);
    println!("Hello, world from {}", user1.email);
}
