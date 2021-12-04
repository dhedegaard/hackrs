use super::{nand::nand, not::not};

pub(crate) const fn xnor(a: bool, b: bool) -> bool {
    nand(nand(not(a), not(b)), nand(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xnor() {
        assert_eq!(xnor(true, true), true);
        assert_eq!(xnor(true, false), false);
        assert_eq!(xnor(false, true), false);
        assert_eq!(xnor(false, false), true);
    }
}
