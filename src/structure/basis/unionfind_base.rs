use crate::structure::basis::Basis;
use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::ptr::write;

pub struct UnionFindBase {
    basis: Vec<usize>,
}

impl Basis for UnionFindBase {
    fn new(n: usize) -> Self {
        UnionFindBase {
            basis: (0..n).collect(),
        }
    }

    fn get_base(&self, u: usize) -> &usize {
        if u != self.basis[u] {
            let base_base = self[self.basis[u]];
            unsafe {
                // Path compression.
                // The vector is invisible to the outside world, therefore it should be safe to mutate it here.
                write((self.basis.as_ptr() as *mut usize).add(u), base_base);
            }
        }
        return &self.basis[u];
    }

    fn set_base(&mut self, u: usize, new_base: usize) {
        self.basis[u] = new_base;
    }
}

impl Index<usize> for UnionFindBase {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_base(index)
    }
}

impl Debug for UnionFindBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            (0..self.basis.len())
                .map(|u| self[u])
                .collect::<Vec<usize>>()
        )
    }
}
