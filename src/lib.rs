pub mod constants;
pub mod alias;
#[cfg(test)]
mod tests {
    pub use super::alias::parse;
    #[test]
    fn parse_test() {
        println!("{}",parse(":flag_ecuador:").unwrap());
        assert!(true);
    }

    #[test]
    fn parse_fail(){
        assert_eq!(None,parse(":hebele:"));
    }
}
