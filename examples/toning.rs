use emojic::flat::HAND_WITH_FINGERS_SPLAYED;
use emojic::flat::PEOPLE_HOLDING_HANDS;
use emojic::flat::RAISED_BACK_OF_HAND;
use emojic::flat::WOMAN_AND_MAN_HOLDING_HANDS;
use emojic::flat::WOMAN_WITH_WHITE_CANE;
use emojic::Tone;

fn main() {
    println!("Plain");
    println!("HAND_WITH_FINGERS_SPLAYED: {}", HAND_WITH_FINGERS_SPLAYED);
    println!("RAISED_BACK_OF_HAND: {}", RAISED_BACK_OF_HAND);

    println!();
    println!("Single toned");
    println!(
        "HAND_WITH_FINGERS_SPLAYED: {}",
        HAND_WITH_FINGERS_SPLAYED.tone(&[Tone::Medium])
    );
    println!(
        "RAISED_BACK_OF_HAND: {}",
        RAISED_BACK_OF_HAND.tone(&[Tone::Medium])
    );

    println!();
    println!(
        "WOMAN_AND_MAN_HOLDING_HANDS: {}",
        WOMAN_AND_MAN_HOLDING_HANDS.tone(&[])
    );
    println!(
        "WOMAN_AND_MAN_HOLDING_HANDS (MEDIUM): {}",
        WOMAN_AND_MAN_HOLDING_HANDS.tone(&[Tone::Medium])
    );
    println!(
        "WOMAN_AND_MAN_HOLDING_HANDS (MEDIUM,MEDIUM): {}",
        WOMAN_AND_MAN_HOLDING_HANDS.tone(&[Tone::Medium, Tone::Medium])
    );
    println!(
        "WOMAN_AND_MAN_HOLDING_HANDS (LIGHT,Dark): {}",
        WOMAN_AND_MAN_HOLDING_HANDS.tone(&[Tone::Light, Tone::Dark])
    );

    println!();
    println!("PEOPLE_HOLDING_HANDS: {}", PEOPLE_HOLDING_HANDS.tone(&[]));
    println!(
        "PEOPLE_HOLDING_HANDS (MEDIUM): {}",
        PEOPLE_HOLDING_HANDS.tone(&[Tone::Medium])
    );
    println!(
        "PEOPLE_HOLDING_HANDS (MEDIUM,MEDIUM): {}",
        PEOPLE_HOLDING_HANDS.tone(&[Tone::Medium, Tone::Medium])
    );
    println!(
        "PEOPLE_HOLDING_HANDS (LIGHT,Dark): {}",
        PEOPLE_HOLDING_HANDS.tone(&[Tone::Light, Tone::Dark])
    );

    println!();
    println!("WOMAN_WITH_WHITE_CANE: {}", WOMAN_WITH_WHITE_CANE.tone(&[]));
    println!(
        "WOMAN_WITH_WHITE_CANE: {}",
        WOMAN_WITH_WHITE_CANE.tone(&[Tone::Medium])
    );
}
