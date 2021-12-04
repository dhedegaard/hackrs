use super::{and::and, not::not, or::or};

pub(crate) const fn xor(a: bool, b: bool) -> bool {
    or(and(a, not(b)), and(not(a), b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!(xor(true, true), false);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(false, false), false);
    }
}
