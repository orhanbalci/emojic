use emojic::grouped::people_and_body::hands;
use emojic::grouped::people_and_body::person_resting;
use emojic::grouped::travel_and_places::place_geographic;

fn main() {
    // Output: 👏🙏🤝👐🤲🙌
    println!(
        "All base emojis of hands: {}",
        hands::base_emojis()
            .map(|e| e.to_string())
            .collect::<String>()
    );

    // Output: 👏👏🏻👏🏼👏🏽👏🏾👏🏿, 🙏🙏🏻🙏🏼🙏🏽🙏🏾🙏🏿, 🤝, 👐👐🏻👐🏼👐🏽👐🏾👐🏿, 🤲🤲🏻🤲🏼🤲🏽🤲🏾🤲🏿, 🙌🙌🏻🙌🏼🙌🏽🙌🏾🙌🏿
    println!(
        "All variants of hands:\n\t{}",
        hands::all_variants()
            .map(|sub| sub.iter().map(|e| e.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join(",\n\t")
    );

    println!();

    // Output: 🛌🧘🛀
    println!(
        "All base emojis of person_resting: {}",
        person_resting::base_emojis()
            .map(|e| e.to_string())
            .collect::<String>()
    );

    // Output: 🛌🛌🏻🛌🏼🛌🏽🛌🏾🛌🏿, 🧘🧘🏻🧘🏼🧘🏽🧘🏾🧘🏿🧘‍♂️🧘🏻‍♂️🧘🏼‍♂️🧘🏽‍♂️🧘🏾‍♂️🧘🏿‍♂️🧘‍♀️🧘🏻‍♀️🧘🏼‍♀️🧘🏽‍♀️🧘🏾‍♀️🧘🏿‍♀️, 🛀🛀🏻🛀🏼🛀🏽🛀🏾🛀🏿
    println!(
        "All variants of person_resting:\n\t{}",
        person_resting::all_variants()
            .map(|sub| sub.iter().map(|e| e.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join(",\n\t")
    );

    println!();

    // Output: 🏖️🏕️🏜️🏝️⛰️🗻🏞️🏔️🌋
    // Notice, in this group there is no difference between `base_emojis()` and `all_variants()`
    println!(
        "All geographic places: {}",
        place_geographic::base_emojis()
            .map(|e| e.to_string())
            .collect::<String>()
    );

    // Outputs the each emoji with its full name and version of introduction
    println!("Geographic places with names and version:");
    for emoji in place_geographic::base_emojis() {
        println!(
            " - {}: {} (since E{})",
            emoji.name, emoji.grapheme, emoji.since
        );
    }
}
