use super::{not::not, or::or};

pub(crate) const fn nor(a: bool, b: bool) -> bool {
    not(or(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nor_test() {
        assert_eq!(nor(true, true), false);
        assert_eq!(nor(true, false), false);
        assert_eq!(nor(false, true), false);
        assert_eq!(nor(false, false), true);
    }
}
