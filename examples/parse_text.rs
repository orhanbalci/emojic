use emojic;

const TEXT: &str = "
Hello :waving_hand:, I am a :technologist:
This :memo: is writen on a :desktop_computer: not with a :pencil:
This :memo: of course is not on a :floppy_disk: but on :computer_disk:
We don't use :file_cabinet: but :file_folder: which can be :open_file_folder:
";

fn main() {
    #[cfg(feature = "alloc")]
    println!("{}", emojic::parse(TEXT));

    #[cfg(not(feature = "alloc"))]
    println!("{}", emojic::EmojiTextParser::new(TEXT));
}
