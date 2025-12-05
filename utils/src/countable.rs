pub trait Countable: Iterator {
    /// Consumes the iterator, returning whether there are more items than some threshold.
    ///
    /// Loops over the iterator, incrementing a count and returning true if that count exceeds the
    /// threshold. If the iterator finishes before that threshold is reached, it returns false.
    fn has_more_than(self, limit: usize) -> bool
    where
        Self: Sized,
    {
        for (i, _) in self.enumerate() {
            if i >= limit {
                return true;
            }
        }

        false
    }

    /// Consumes the iterator, returning whether there are at least as many items as some
    /// threshold.
    ///
    /// Loops over the iterator, incrementing a count and returning true if that count equals or
    /// exceeds the threshold. If the iterator finishes before that threshold is reached, it
    /// returns whether the limit equals 0.
    fn has_at_least(self, limit: usize) -> bool
    where
        Self: Sized,
    {
        for (i, _) in self.enumerate() {
            if i + 1 >= limit {
                return true;
            }
        }

        limit == 0
    }

    /// Consumes the iterator, returning whether there are fewer items than some threshold.
    ///
    /// Loops over the iterator, incrementing a count and returning false if that count equals or
    /// exceeds the threshold. If the iterator finishes before that threshold is reached, it
    /// returns whether the limit does not equal 0.
    fn has_fewer_than(self, limit: usize) -> bool
    where
        Self: Sized,
    {
        for (i, _) in self.enumerate() {
            if i + 1 >= limit {
                return false;
            }
        }

        limit != 0
    }

    /// Consumes the iterator, returning whether there are at most as many items as some threshold.
    ///
    /// Loops over the iterator, incrementing a count and returning false if that count exceeds the
    /// threshold. If the iterator finishes before that threshold is reached, it returns true.
    fn has_at_most(self, limit: usize) -> bool
    where
        Self: Sized,
    {
        for (i, _) in self.enumerate() {
            if i >= limit {
                return false;
            }
        }

        true
    }
}

impl<I: Iterator> Countable for I {}

#[cfg(test)]
mod tests {
    use super::Countable;

    #[test]
    fn has_more_than_0() {
        let v = [1, 2, 3];
        assert!(v.iter().has_more_than(0));
    }

    #[test]
    fn has_more_than_1() {
        let v = [1, 2, 3];
        assert!(v.iter().has_more_than(1));
    }

    #[test]
    fn has_more_than_2() {
        let v = [1, 2, 3];
        assert!(v.iter().has_more_than(2));
    }

    #[test]
    fn has_more_than_3() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_more_than(3));
    }

    #[test]
    fn has_more_than_4() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_more_than(4));
    }

    #[test]
    fn has_at_least_0() {
        let v = [1, 2, 3];
        assert!(v.iter().has_at_least(0));
    }

    #[test]
    fn has_at_least_1() {
        let v = [1, 2, 3];
        assert!(v.iter().has_at_least(1));
    }

    #[test]
    fn has_at_least_2() {
        let v = [1, 2, 3];
        assert!(v.iter().has_at_least(2));
    }

    #[test]
    fn has_at_least_3() {
        let v = [1, 2, 3];
        assert!(v.iter().has_at_least(3));
    }

    #[test]
    fn has_at_least_4() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_at_least(4));
    }

    #[test]
    fn has_fewer_than_0() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_fewer_than(0));
    }

    #[test]
    fn has_fewer_than_1() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_fewer_than(1));
    }

    #[test]
    fn has_fewer_than_2() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_fewer_than(2));
    }

    #[test]
    fn has_fewer_than_3() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_fewer_than(3));
    }

    #[test]
    fn has_fewer_than_4() {
        let v = [1, 2, 3];
        assert!(v.iter().has_fewer_than(4));
    }

    #[test]
    fn has_at_most_0() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_at_most(0));
    }

    #[test]
    fn has_at_most_1() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_at_most(1));
    }

    #[test]
    fn has_at_most_2() {
        let v = [1, 2, 3];
        assert!(!v.iter().has_at_most(2));
    }

    #[test]
    fn has_at_most_3() {
        let v = [1, 2, 3];
        assert!(v.iter().has_at_most(3));
    }

    #[test]
    fn has_at_most_4() {
        let v = [1, 2, 3];
        assert!(v.iter().has_at_most(4));
    }

    #[test]
    fn empty_has_more_than_0() {
        let v: [i64; 0] = [];
        assert!(!v.iter().has_more_than(0));
    }

    #[test]
    fn empty_has_fewer_than_0() {
        let v: [i64; 0] = [];
        assert!(!v.iter().has_fewer_than(0));
    }

    #[test]
    fn empty_has_at_most_0() {
        let v: [i64; 0] = [];
        assert!(v.iter().has_at_most(0));
    }

    #[test]
    fn empty_has_at_least_0() {
        let v: [i64; 0] = [];
        assert!(v.iter().has_at_least(0));
    }

    #[test]
    fn empty_has_more_than_1() {
        let v: [i64; 0] = [];
        assert!(!v.iter().has_more_than(1));
    }

    #[test]
    fn empty_has_fewer_than_1() {
        let v: [i64; 0] = [];
        assert!(v.iter().has_fewer_than(1));
    }

    #[test]
    fn empty_has_at_most_1() {
        let v: [i64; 0] = [];
        assert!(v.iter().has_at_most(1));
    }

    #[test]
    fn empty_has_at_least_1() {
        let v: [i64; 0] = [];
        assert!(!v.iter().has_at_least(1));
    }
}
