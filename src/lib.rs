pub mod constants;
pub mod alias;
#[cfg(test)]
mod tests {
    pub use super::alias;
    #[test]
    fn parse_test() {
        println!("{}",alias::parse(":+1:").unwrap());
        assert!(true);
    }
}
