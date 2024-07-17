pub mod observer_base;
pub mod unionfind_base;

pub use observer_base::ObserverBase;
use std::fmt::Debug;
pub use unionfind_base::UnionFindBase;

use std::ops::Index;

pub trait Basis: Index<usize> + Debug
where
    <Self as Index<usize>>::Output: PartialEq,
{
    fn new(n: usize) -> Self;
    fn get_base(&self, u: usize) -> &usize;
    fn set_base(&mut self, u: usize, new_base: usize);
    fn same_base(&self, u: usize, v: usize) -> bool {
        self[u] == self[v]
    }
}

#[cfg(test)]
mod test_base {
    use super::*;
    use crate::structure::basis::observer_base::ObserverBase;
    use crate::structure::basis::unionfind_base::UnionFindBase;
    use std::fmt::Debug;

    fn test_basis<B: Basis>()
    where
        <B as Index<usize>>::Output: PartialEq,
    {
        let mut base = B::new(10);

        assert!(!base.same_base(0, 1));
        assert!(!base.same_base(2, 3));

        base.set_base(1, 0);
        assert!(base.same_base(0, 1));
        assert!(base.same_base(1, 0));
        assert!(!base.same_base(2, 3));

        base.set_base(3, 4);
        base.set_base(5, 4);
        base.set_base(7, 8);
        base.set_base(9, 5);

        assert!(!base.same_base(9, 0));
        assert!(!base.same_base(8, 5));
        assert!(base.same_base(9, 4));
        assert!(base.same_base(3, 5));

        base = B::new(15);
        assert!(!base.same_base(4, 5));
        assert!(!base.same_base(3, 6));
        assert!(base.same_base(10, 10));

        base.set_base(10, 11);
        base.set_base(12, 11);
        assert!(base.same_base(10, 12));
        assert!(base.same_base(12, 11));

        base.set_base(4, 5);
        base.set_base(6, 4);
        base.set_base(5, 11);
        assert!(base.same_base(4, 5));
        assert!(base.same_base(4, 11));
        assert!(base.same_base(4, 12));
        assert!(base.same_base(6, 10));
    }

    #[test]
    fn test_unionfind_base() {
        test_basis::<UnionFindBase>();
    }
    #[test]
    fn test_observer_base() {
        test_basis::<ObserverBase>();
    }
}
