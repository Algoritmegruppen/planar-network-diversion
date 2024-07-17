use crate::structure::graph::edge::Edge;
use crate::structure::graph::planar_edge::{intersect, PlanarEdge, PrePlanarEdge};
use crate::structure::graph::point::{compare_edges_clockwise, Point};
use crate::structure::graph::simple_graph_strategy::{SimpleGraphStrategy, SumWeights};
use crate::structure::graph::undirected_graph::UndirectedGraph;
use crate::structure::weight::Weight;
use crate::utility::misc::{debug, repeat};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct PlanarGraph<W: Weight> {
    real: UndirectedGraph<W, PlanarEdge<W>>,
    dual: UndirectedGraph<W, PlanarEdge<W>>,
}

impl<W: Weight> PlanarGraph<W> {
    pub fn real(&self) -> &UndirectedGraph<W, PlanarEdge<W>> {
        &self.real
    }
    pub fn dual(&self) -> &UndirectedGraph<W, PlanarEdge<W>> {
        &self.dual
    }
    pub fn n(&self) -> usize {
        self.real.n()
    }
    pub fn m(&self) -> usize {
        self.real.m()
    }
    pub fn f(&self) -> usize {
        self.dual.n()
    }
    pub fn parse<S: SimpleGraphStrategy>(
        str: &str,
        assert_planarity: bool,
    ) -> Result<Self, &'static str> {
        let mut ls = str
            .lines()
            .map(str::trim)
            .filter(|&l| l.len() > 0 && !l.starts_with("%"));
        let mut row1 = ls
            .next()
            .ok_or("Could not find the first row")?
            .split(' ')
            .map(usize::from_str);
        let n = row1
            .next()
            .ok_or("Could not find n")?
            .or(Err("Could not parse n"))?;
        let m = row1
            .next()
            .ok_or("Could not find m")?
            .or(Err("Could not parse m"))?;
        let mut pre = PrePlanarGraph::empty(n, assert_planarity);

        for _ in 0..n {
            let mut ws = ls
                .next()
                .ok_or("Expected another vertex here, but got nothing")?
                .split(' ');
            let id = ws
                .next()
                .ok_or("Could not find the id")?
                .parse()
                .or(Err("Could not parse the id"))?;
            let x = ws
                .next()
                .ok_or("Could not find the x coordinate")?
                .parse()
                .or(Err("Could not parse the x coordinate"))?;
            let y = ws
                .next()
                .ok_or("Could not find the y coordinate")?
                .parse()
                .or(Err("Could not parse the y coordinate"))?;
            pre.add_vertex(id, Point::new(x, y));
        }
        for _ in 0..m {
            pre.add_edge::<S>(
                ls.next()
                    .ok_or("Expected another edge here, but got nothing")?
                    .parse()?,
            );
        }
        Ok(pre.planarize()?)
    }
}

struct PrePlanarGraph<W: Weight> {
    graph: UndirectedGraph<W, PrePlanarEdge<W>>,
    points: Vec<Option<Point>>,
    assert_planarity: bool,
}

impl<W: Weight> PrePlanarGraph<W> {
    pub fn empty(n: usize, assert_planarity: bool) -> Self {
        PrePlanarGraph {
            graph: UndirectedGraph::new(n),
            points: repeat(n, None),
            assert_planarity,
        }
    }
    pub fn add_vertex(&mut self, i: usize, u: Point) {
        self.points[i] = Some(u);
    }

    pub fn add_edge<S: SimpleGraphStrategy>(&mut self, x: PrePlanarEdge<W>) {
        let (u, v, e) = if self.graph.adj_list[x.from].len() < self.graph.adj_list[x.to].len() {
            (x.from(), x.to(), x)
        } else {
            (x.to(), x.from(), x.reverse())
        };
        if let Some(i) = self.graph.adj_list[u].iter().position(|x| x.to == v) {
            let b = e.reverse();
            self.graph.adj_list[u][i] = S::combine(e, self.graph.adj_list[u][i].clone());
            let j = self.graph.adj_list[v]
                .iter()
                .position(|v| v.to == u)
                .expect("Uhm, looks like we have a uni-directional edge here");
            self.graph.adj_list[v][j] = S::combine(b, self.graph.adj_list[v][j].clone());
        } else {
            self.graph.add_edge(e);
        }
    }

    pub fn planarize(mut self) -> Result<PlanarGraph<W>, &'static str> {
        let mut points = Vec::new();
        for p in &self.points {
            points.push(p.clone().ok_or("Not all points have been defined")?);
        }
        if self.assert_planarity {
            self.assert_planarity(&points)?;
        }

        self.sort_edges(&points);
        let f = self.determine_faces()?;

        let mut real = UndirectedGraph::new(self.graph.n());
        let mut dual = UndirectedGraph::new(f);
        self.graph.adj_list.iter().for_each(|xs| {
            xs.iter().filter(|e| e.from() < e.to()).for_each(|e| {
                let p = e.planarize();
                let b = p.rotate_right();
                real.add_edge(p);
                dual.add_edge(b);
            })
        });

        Ok(PlanarGraph { real, dual })
    }

    fn sort_edges(&mut self, points: &Vec<Point>) {
        for u in 0..self.graph.n() {
            self.graph.adj_list[u].sort_by(compare_edges_clockwise(&points[u], &points));
        }
    }
    fn determine_faces(&mut self) -> Result<usize, &'static str> {
        let n = self.graph.n();
        let adj_list = &mut self.graph.adj_list;
        let adj_list_copy = adj_list.clone();
        let mut current_face = 0;
        for start_vertex in 0..n {
            for mut curr_line_id in 0..adj_list[start_vertex].len() {
                let mut curr_line = &adj_list_copy[start_vertex][curr_line_id];
                if adj_list[start_vertex][curr_line_id].left.is_none() {
                    loop {
                        adj_list[curr_line.from][curr_line_id].left = Some(current_face);
                        let id = adj_list_copy[curr_line.to]
                            .iter()
                            .position(|e| e.to == curr_line.from)
                            .expect("Couldn't find the reverse edge");
                        adj_list[curr_line.to][id].right = Some(current_face);
                        curr_line_id = (id + 1) % adj_list[curr_line.to].len();
                        curr_line = &adj_list_copy[curr_line.to][curr_line_id];

                        if curr_line.from == start_vertex {
                            break;
                        }
                    }
                    current_face += 1;
                }
            }
        }

        for u in 0..n {
            for e in &adj_list[u] {
                if e.left.is_none() || e.right.is_none() {
                    return Err("Not all edges found both a left and right region!");
                }
            }
        }
        if self.graph.m() > n + current_face || n + current_face - self.graph.m() != 2 {
            debug(format!(
                "n = {}, m = {}, f = {}",
                self.graph.n(),
                self.graph.m(),
                current_face
            ));
            debug(format!(
                "We should have had {} - {} + 2 = {} regions, but we found {}.",
                self.graph.m(),
                n,
                self.graph.m() - self.graph.n() + 2,
                current_face
            ));
            debug(format!(
                "Either we don't have the correct faces, or Euler's formula is wrong :thinkin:"
            ));
            if self.assert_planarity {
                panic!("Incorrect number of regions compared to vertices and edges!");
            }
        }
        Ok(current_face)
    }

    fn assert_planarity(&self, points: &Vec<Point>) -> Result<(), &'static str> {
        let mut errors = 0;
        let edges = self.graph.edges();
        for i in 0..edges.len() {
            let ab = &edges[i];
            if ab.from() > ab.to() {
                continue;
            }
            for j in i + 1..edges.len() {
                let cd = &edges[j];
                if cd.from() < cd.to() && ab != &cd.reverse() && intersect(&points, ab, cd) {
                    if errors == 0 {
                        debug("    This cannot be a straight-line embedding, here are some pairs of edges that intersect: ".to_string());
                    }
                    if errors < 10 {
                        debug(format!(
                            "        {}  x  {}",
                            ab.format_with_coords(&points),
                            cd.format_with_coords(&points)
                        ));
                    }
                    errors += 1;
                }
            }
        }
        if errors == 0 {
            Ok(())
        } else {
            Err("This is not a straight-line embedding :(")
        }
    }
}

impl<W: Weight> Debug for PlanarGraph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PlanarGraph(n = {}, m = {}, f = {}):\n",
            self.n(),
            self.m(),
            self.f()
        )?;
        write!(f, "Real part:\n")?;
        self.real.fmt(f)?;
        write!(f, "Dual part:\n")?;
        self.dual.fmt(f)?;
        Ok(())
    }
}

impl<W: Weight> FromStr for PlanarGraph<W> {
    type Err = &'static str;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::parse::<SumWeights>(str, false)
    }
}

mod test_planar_graph {
    use crate::structure::graph::planar_graph::PlanarGraph;
    use crate::structure::graph::simple_graph_strategy::SumWeights;
    use crate::utility::misc::debug;
    use std::fs::read_to_string;

    fn _parse(folder: &str, name: &str) -> PlanarGraph<f64> {
        debug(format!("Attempting to parse {}...", name));
        let input =
            read_to_string(["data/planar_graphs/", folder, "/", name, "/", name, ".in"].concat())
                .expect("No graph found");
        PlanarGraph::parse::<SumWeights>(&input, true)
            .unwrap_or_else(|err| panic!("Could not parse the graph: {}", err))
    }

    #[test]
    fn test_small_planar1() {
        let planar = _parse("small_planar_graphs", "small_planar1");

        assert!(planar.dual().is_adjacent(0, 0));
        assert!(planar.dual().is_adjacent(0, 1));
        assert!(planar.dual().is_adjacent(1, 3));
        assert!(planar.dual().is_adjacent(1, 2));
        assert!(planar.dual().is_adjacent(2, 4));
        assert!(planar.dual().is_adjacent(3, 4));
        assert!(planar.dual().is_adjacent(3, 5));
        assert!(!planar.dual().is_adjacent(2, 5));
        assert!(!planar.dual().is_adjacent(2, 6));
        assert!(!planar.dual().is_adjacent(1, 5));
        assert!(!planar.dual().is_adjacent(2, 6));
    }

    #[test]
    fn test_small_planar2() {
        let planar = parse("small_planar_graphs", "small_planar2");

        assert!(planar.dual().is_adjacent(0, 0));
        assert!(planar.dual().is_adjacent(0, 1));
        assert!(planar.dual().is_adjacent(1, 2));
        assert!(planar.dual().is_adjacent(1, 6));
        assert!(planar.dual().is_adjacent(5, 6));
        assert!(planar.dual().is_adjacent(4, 5));
        assert!(planar.dual().is_adjacent(4, 7));
        assert!(planar.dual().is_adjacent(4, 2));
        assert!(planar.dual().is_adjacent(7, 8));
        assert!(planar.dual().is_adjacent(8, 0));

        assert!(!planar.dual().is_adjacent(1, 7));
        assert!(!planar.dual().is_adjacent(2, 5));
        assert!(!planar.dual().is_adjacent(5, 3));
        assert!(!planar.dual().is_adjacent(5, 0));
        assert!(!planar.dual().is_adjacent(5, 5));
    }

    #[test]
    fn assert_small_planarity() {
        for name in [
            "small_planar1",
            "small_planar2",
            "small_planar3",
            "small_planar4",
        ] {
            parse("small_planar_graphs", name);
        }
    }
    #[ignore = "The graphs are too large and take too much time to test over again each time."]
    #[test]
    fn assert_large_planarity() {
        for name in [
            // "CityOfOldenburg",
            "CaliforniaRoadNetwork",
            // "CityOfSanJoaquinCounty",
            // "SanFranciscoRoadNetwork",
        ] {
            parse("real_planar_graphs", name);
        }
    }
}
