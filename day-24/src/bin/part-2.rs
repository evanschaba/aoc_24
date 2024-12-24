pub fn add(a: u8, b: u8) -> u8 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(1, 1), 2);
    }
}

fn main() {}
