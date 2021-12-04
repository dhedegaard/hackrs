use super::nand::nand;

pub(crate) const fn and(a: bool, b: bool) -> bool {
    nand(nand(a, b), nand(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(and(true, true), true);
        assert_eq!(and(true, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(false, false), false);
    }
}
