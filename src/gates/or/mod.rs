use super::{nand::nand, not::not};

pub(crate) const fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_or() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }
}
