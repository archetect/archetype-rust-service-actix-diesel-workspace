use log::{trace};

pub fn get_greeting() -> &'static str {
    trace!("Preparing greeting");
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello, World!");
    }
}
