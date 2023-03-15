
mod handshake;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u16 = input.trim().parse().unwrap_or_default();
    let secret_handshake = handshake::Handshake::parse(n);
    println!("{:?}", secret_handshake.unwrap_or_default());
}

#[cfg(test)]
mod test {
    use super::handshake::{Handshake ,Actions::*};

    #[test]
    fn test_handshake_1() {
        let handshake = Handshake::parse(1);
        assert_eq!(handshake, Some(vec![Wink]));
    }

    #[test]
    fn test_handshake_9() {
        let handshake = Handshake::parse(9);
        assert_eq!(handshake, Some(vec![Wink, Jump]));
    }

    #[test]
    fn test_handshake_32() {
        let handshake = Handshake::parse(32);
        assert_eq!(handshake, None);
    }
}
