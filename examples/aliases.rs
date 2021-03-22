fn print_alias(s: &str) {
    println!("{} => {}", s, emojic::parse_alias(s).unwrap());
}

fn main() {
    print_alias(":+1:"); // 👍
    print_alias(":alien_monster:"); // 👾
    print_alias(":joy:"); // 😂
    print_alias(":rocket:"); // 🚀
}
