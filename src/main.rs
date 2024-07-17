use std::fs::read_to_string;

pub mod algorithm;
pub mod structure;
pub mod utility;

use planar_network_diversion::algorithm::network_diversion::network_diversion;
use planar_network_diversion::structure::graph::planar_graph::PlanarGraph;

use std::env;
use std::time::Instant;

use planar_network_diversion::structure::graph::edge::Edge;

fn parse_graph(filename: &str) -> PlanarGraph<f64> {
    let graph: PlanarGraph<f64> = read_to_string(filename)
        .expect("Could not find the graph")
        .parse()
        .expect("Could not read the graph");
    graph
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        eprintln!("Usage: {} <file_name> <s> <t> <b1> <bt>", args[0]);
        std::process::exit(1);
    }

    let fname = &args[1];
    let s: usize = args[2].parse().expect("s must be an integer");
    let t: usize = args[3].parse().expect("t must be an integer");
    let b1: usize = args[4].parse().expect("b1 must be an integer");
    let b2: usize = args[5].parse().expect("b2 must be an integer");

    let graph = parse_graph(fname);

    let start_time = Instant::now();
    if let Some((_, v)) = network_diversion(&graph, s, t, (b1, b2)) {
        let secs = start_time.elapsed().as_secs_f64();
        println!("{:.3} s", secs);
        for i in 0..v.len() {
            println!("{},{}", v[i].from(), v[i].to());
        }
    } else {
        let secs = start_time.elapsed().as_secs_f64();
        println!("{:.3} s: WARN: No cut found", secs);
    }
}
