use emojic::flat::FAMILY;
use emojic::Gender;
use emojic::Pair;

fn main() {
    println!("Default: {}", FAMILY); // 👪
    println!("From pairs: {}", FAMILY.family((Pair::Mixed, Pair::Males))); // 👨‍👩‍👦‍👦
    println!(
        "From gender: {}",
        FAMILY.family((Gender::Male, Gender::Female))
    ); // 👨‍👧
    println!(
        "From gender and pair: {}",
        FAMILY.family(Gender::Female.with_children(Pair::Mixed))
    ); // 👩‍👧‍👦
    println!(
        "From pair and gender: {}",
        FAMILY.family(Pair::Mixed.with_children(Gender::Female))
    ); // 👨‍👩‍👧
}
