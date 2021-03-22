use emojic::flat::PERSON;
use emojic::Gender;
use emojic::Hair;
use emojic::Tone;

fn main() {
    println!("Default: {}", PERSON); // 🧑
    println!("With tone: {}", PERSON.tone(Tone::Dark)); // 🧑🏿
    println!("With hair: {}", PERSON.hair(Hair::Red)); // 🧑‍🦰
    println!("With gender: {}", PERSON.gender(Gender::Female)); // 👩

    println!(
        "With beard & man: {}",
        PERSON.hair(Hair::Beard).gender(Gender::Male)
    ); // 🧔‍♂️
    println!(
        "With light & woman: {}",
        PERSON.gender(Gender::Female).tone(Tone::Light)
    ); // 👩🏻
    println!(
        "With bald & medium: {}",
        PERSON.hair(Hair::Bald).tone(Tone::Medium)
    ); // 🧑🏽‍🦲

    println!(
        "With blond & man & dark: {}",
        PERSON
            .hair(Hair::Blond)
            .gender(Gender::Male)
            .tone(Tone::Dark)
    ); // 👱🏿‍♂️

    // Would not compile
    //println!("With tone: {}", PERSON.tone(Tone::Dark).tone(Tone::Light));
    //println!("With tone: {}", ALIEN.tone(Tone::Dark));
}
