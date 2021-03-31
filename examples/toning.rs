use emojic::flat::HAND_WITH_FINGERS_SPLAYED;
use emojic::flat::PERSON_HOLDING_HANDS;
use emojic::flat::RAISED_BACK_OF_HAND;
use emojic::Tone;

fn main() {
    println!("Plain");
    println!("HAND_WITH_FINGERS_SPLAYED: {}", HAND_WITH_FINGERS_SPLAYED); // 🖐️
    println!("RAISED_BACK_OF_HAND: {}", RAISED_BACK_OF_HAND); // 🤚

    println!();
    println!("Toned");
    println!(
        "HAND_WITH_FINGERS_SPLAYED: {}",
        HAND_WITH_FINGERS_SPLAYED.tone(Tone::Medium) // 🖐🏽
    );
    println!(
        "RAISED_BACK_OF_HAND: {}",
        RAISED_BACK_OF_HAND.tone(Tone::Medium) // 🤚🏽
    );

    println!();
    println!(
        "PERSON_HOLDING_HANDS: {}",
        PERSON_HOLDING_HANDS // 🧑‍🤝‍🧑
    );
    println!(
        "PERSON_HOLDING_HANDS (Medium): {}",
        PERSON_HOLDING_HANDS.tone(Tone::Medium) // 🧑🏽‍🤝‍🧑🏽
    );
    println!(
        "PERSON_HOLDING_HANDS (Medium,Medium): {}",
        PERSON_HOLDING_HANDS.tone((Tone::Medium, Tone::Medium)) // 🧑🏽‍🤝‍🧑🏽
    );
    println!(
        "PERSON_HOLDING_HANDS (Light,Dark): {}",
        PERSON_HOLDING_HANDS.tone((Tone::Light, Tone::Dark)) // 🧑🏻‍🤝‍🧑🏿
    );
    println!(
        "PERSON_HOLDING_HANDS (Dark,Light): {}",
        PERSON_HOLDING_HANDS.tone((Tone::Dark, Tone::Light)) // 🧑🏿‍🤝‍🧑🏻
    );
}
