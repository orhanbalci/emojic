use emojic::flat::FAMILY;
use emojic::flat::FAMILYE;
use emojic::Gender;
use emojic::Pair;

fn main() {
    println!("Default: {}", FAMILYE); // 👪
    println!("From pairs: {}", FAMILY.gender((Pair::Mixed, Pair::Males))); // 👨‍👩‍👦‍👦
    println!(
        "From gender: {}",
        FAMILY.gender((Gender::Male, Gender::Female))
    ); // 👨‍👧
    println!(
        "From gender and pair: {}",
        FAMILY.gender(Gender::Female.with_children(Pair::Mixed))
    ); // 👩‍👧‍👦
    println!(
        "From pair and gender: {}",
        FAMILY.gender(Pair::Mixed.with_children(Gender::Female))
    ); // 👨‍👩‍👧
}
