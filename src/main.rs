// Needs to be able to set size, connectedness and locality (how far away from the current nodes value may a node we are connecting to be)

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Count of vertexes in the generated graph
    #[arg(short, long)]
    size: usize,

    /// The probability a node connects to each other reachable node
    #[arg(short, long)]
    connectedness: f64,

    /// The difference in node id with which a given node can still connect.
    #[arg(short, long)]
    locality: usize,

    /// If set, the locality wraps around such that node 1 can reach node <size>
    #[arg(short, long)]
    wraparound: bool,

    /// Sets the graph to be directed
    #[arg(short, long)]
    directed: bool,
}

fn main() {
    let mut args = Args::parse();
    if args.locality == 0 {
        args.locality = args.size;
    }
    let args = args;

    let graph = generate_graph(
        args.size,
        args.connectedness,
        args.locality,
        args.directed,
        args.wraparound,
    );

    let edges = graph
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .flat_map(|(i, v)| {
            v.iter()
                .filter(move |e| if !args.directed { **e > i } else { true })
        })
        .count();
    eprintln!("Edge count: {edges}");

    let actual_locality = graph
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1, v))
        .flat_map(|(v, es)| es.iter().map(move |e| v.max(*e) - v.min(*e)))
        .max()
        .unwrap_or(0);
    eprintln!("Actual locality: {actual_locality}");

    for (i, n) in graph.into_iter().enumerate().map(|(i, n)| (i + 1, n)) {
        print!("{} 1", i);
        for connection in n.into_iter().map(|i| i + 1) {
            print!(" {connection}");
        }
        println!(" 0");
    }
}

fn generate_graph(
    size: usize,
    connectedness: f64,
    locality: usize,
    directed: bool,
    locality_wrapping: bool,
) -> Vec<Vec<usize>> {
    use rand::prelude::*;
    let mut graph: Vec<Vec<usize>> = vec![vec![]; size];

    let mut rng = thread_rng();
    let mut rng = rand::distributions::Uniform::new_inclusive(0.0, 1.0).sample_iter(&mut rng);

    let mut roll = move || rng.next().unwrap() <= connectedness;

    for i in 0..size {
        let sec_1 = (i + 1)..(i + locality + 1).min(size);
        let sec_2 = if locality_wrapping {
            (i + locality + 1).max(size - locality + i)..(size)
        } else {
            0..0 // NOTE: Ranges are empty if start >= end
        };
        for j in sec_1.chain(sec_2) {
            let connect = roll();
            if connect {
                graph[i].push(j);
            }
            if !directed {
                if connect {
                    graph[j].push(i);
                }
            } else {
                if roll() {
                    graph[j].push(i);
                }
            }
        }
    }
    graph
}
