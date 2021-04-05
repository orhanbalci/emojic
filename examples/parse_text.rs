const TEXT: &str = "
Hello :waving_hand:, I am a :technologist:
This :memo: is written on a :desktop_computer: not with a :pencil:
This :memo: of course is not on a :floppy_disk: but on :computer_disk:
Stored in a :file_folder: that can be :open_file_folder: instead of a :file_cabinet:
";

fn main() {
    #[cfg(feature = "alloc")]
    println!("{}", emojic::text::parse_text(TEXT));

    #[cfg(not(feature = "alloc"))]
    println!("{}", emojic::text::EmojiTextParser::new(TEXT));
}
