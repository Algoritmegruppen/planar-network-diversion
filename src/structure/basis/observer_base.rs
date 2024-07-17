use crate::structure::basis::Basis;
use crate::utility::misc::repeat;
use std::fmt::{Debug, Formatter};
use std::ops::Index;

pub struct ObserverBase {
    basis: Vec<usize>,
    dependents: Vec<Option<Vec<usize>>>,
}

impl Basis for ObserverBase {
    fn new(n: usize) -> Self {
        ObserverBase {
            basis: (0..n).collect(),
            dependents: repeat(n, None),
        }
    }
    fn get_base(&self, u: usize) -> &usize {
        &self.basis[u]
    }
    fn set_base(&mut self, u: usize, new_base: usize) {
        if u == new_base {
            return;
        }
        if new_base != self.basis[new_base] {
            return self.set_base(u, self.basis[new_base]);
        }
        if self.dependents[new_base].is_none() {
            self.dependents[new_base] = Some(Vec::new());
        }
        let u_deps = std::mem::replace(&mut self.dependents[u], None);
        self.dependents[new_base].as_mut().map(|xs| {
            self.basis[u] = new_base;
            xs.push(u);
            if let Some(ys) = u_deps {
                ys.iter().for_each(|v| self.basis[*v] = new_base);
                xs.extend(ys);
            }
        });
    }
}

impl Index<usize> for ObserverBase {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.get_base(index)
    }
}

impl Debug for ObserverBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.basis.len())
                .filter(|u| self.dependents[*u].is_some())
                .map(|u| format!("{}: {:?}", u, self.dependents[u].clone().unwrap()))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
