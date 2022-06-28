pub struct Config {
    pub amount_of_dsa: u32,
}

impl Config {
    pub fn new(amount_of_dsa: u32) -> Self {
        Self { amount_of_dsa }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
