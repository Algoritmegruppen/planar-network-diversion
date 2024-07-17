use crate::structure::graph::edge::Edge;
use crate::structure::weight::Weight;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::str;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub struct UndirectedGraph<W, E>
where
    W: Weight,
    E: Edge<W>,
{
    pub(in crate::structure::graph) adj_list: Vec<Vec<E>>,
    n: usize,
    m: usize,
    _marker: PhantomData<W>,
}
impl<W: Weight, E: Edge<W>> UndirectedGraph<W, E> {
    pub fn new(n: usize) -> Self {
        UndirectedGraph {
            adj_list: (0..n).map(|_| Vec::new()).collect(),
            n,
            m: 0,
            _marker: PhantomData::default(),
        }
    }
    pub fn n(&self) -> usize {
        self.n
    }
    pub fn m(&self) -> usize {
        self.m
    }
    pub fn add_edge(&mut self, e: E) {
        let b = e.reverse();
        self.adj_list[e.from()].push(e);
        self.adj_list[b.from()].push(b);
        self.m += 1;
    }
    pub fn is_adjacent(&self, u: usize, v: usize) -> bool {
        let (p, q) = if self.adj_list[u].len() < self.adj_list[v].len() {
            (u, v)
        } else {
            (v, u)
        };
        self.adj_list[p].iter().find(|e| e.to() == q).is_some()
    }
    pub fn find_edges(&self, u: usize, v: usize) -> Vec<E> {
        self.adj_list[u]
            .clone()
            .into_iter()
            .filter(|e| e.to() == v)
            .collect()
    }
    pub fn delete_edges(&mut self, r: &Vec<E>) {
        for e in r {
            self.adj_list[e.from()].retain(|f| f.to() != e.to());
            self.adj_list[e.to()].retain(|f| f.to() != e.from())
        }
    }
    #[allow(non_snake_case)]
    pub fn N(&self, u: usize) -> &Vec<E> {
        &self[&u]
    }
    #[allow(non_snake_case)]
    pub fn V(&self) -> impl Iterator<Item = usize> {
        self.vertices()
    }
    pub fn vertices(&self) -> impl Iterator<Item = usize> {
        0..self.n
    }
    pub fn edges(&self) -> Vec<E> {
        self.adj_list.clone().into_iter().flatten().collect()
    }
}

impl<W: Weight, E: Edge<W>> From<String> for UndirectedGraph<W, E> {
    fn from(value: String) -> Self {
        Self::from_str(value.as_str()).expect(
            format!(
                "Could not parse the following as an UndirectedGraph: \n{}",
                &value[..50.min(value.len() - 1)]
            )
            .as_str(),
        )
    }
}

impl<W: Weight, E: Edge<W>> FromStr for UndirectedGraph<W, E> {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut ls = str
            .lines()
            .map(str::trim)
            .filter(|&l| l.len() > 0 && !l.starts_with("%"));
        let row1 = ls.next().ok_or(
            "Expected an integer denoting the number of vertices, but found nothing!".to_owned(),
        )?;
        let n = row1
            .split(' ')
            .next()
            .unwrap()
            .parse()
            .or(Err(format!("Could not parse '{}' as n", row1)))?;
        let mut ret = UndirectedGraph::new(n);
        for row in ls {
            let p: E = row
                .parse()
                .or_else(|_| Err(format!("Could not parse the row: {}", row)))?;
            ret.add_edge(p);
        }
        Ok(ret)
    }
}

impl<W: Weight, E: Edge<W>> Index<&usize> for UndirectedGraph<W, E> {
    type Output = Vec<E>;
    fn index(&self, u: &usize) -> &Self::Output {
        &self.adj_list[*u]
    }
}
impl<W: Weight, E: Edge<W>> Index<usize> for UndirectedGraph<W, E> {
    type Output = Vec<E>;
    fn index(&self, u: usize) -> &Self::Output {
        &self[&u]
    }
}

impl<W: Weight, E: Edge<W>> IndexMut<&usize> for UndirectedGraph<W, E> {
    fn index_mut(&mut self, u: &usize) -> &mut Self::Output {
        &mut self.adj_list[*u]
    }
}
impl<W: Weight, E: Edge<W>> IndexMut<usize> for UndirectedGraph<W, E> {
    fn index_mut(&mut self, u: usize) -> &mut Self::Output {
        &mut self[&u]
    }
}

impl<W, E> Debug for UndirectedGraph<W, E>
where
    W: Weight,
    E: Edge<W> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str(format!("UndirectedGraph(n = {}, m = {}):\n", self.n, self.m).as_str());
        for u in self.vertices().take(15) {
            ret.push_str(format!("  N({}) = {:?}\n", u, self[&u]).as_str());
        }
        write!(f, "{}", ret)
    }
}
