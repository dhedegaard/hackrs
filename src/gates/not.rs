use super::*;

pub(crate) const fn not(a: bool) -> bool {
    nand::nand(a, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not() {
        assert_eq!(not(true), false);
        assert_eq!(not(false), true);
    }
}
