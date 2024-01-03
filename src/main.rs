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

    /// The difference in node id with which a given node can still connect. This wraps, such that node 1 can always connect to node _size_.
    /// A value of zero or greater than size is interpreted as no restriction.
    #[arg(short, long)]
    locality: usize,

    /// Sets the graph to be directed
    #[arg(short, long)]
    directed: bool,
}

fn main() {
    use rand::prelude::*;
    let mut args = Args::parse();
    if args.locality == 0 {
        args.locality = args.size;
    }
    let args = args;

    let mut graph: Vec<Vec<usize>> = vec![vec![]; args.size];

    let mut rng = thread_rng();
    let mut rng = rand::distributions::Uniform::new_inclusive(0.0, 1.0).sample_iter(&mut rng);

    let mut roll = move || rng.next().unwrap() <= args.connectedness;

    for i in 0..args.size {
        let sec_1 = (i + 1)..(i + 1 + args.locality).min(args.size);
        let sec_2 = (i + args.locality + 1).max(args.size - args.locality + i)..(args.size);
        for j in dbg!(sec_1).chain(dbg!(sec_2)) {
            let connect = roll();
            if connect {
                graph[i].push(j);
            }
            if args.directed {
                if roll() {
                    graph[j].push(i);
                }
            } else if connect {
                graph[j].push(i);
            }
        }
    }

    for (i, n) in graph.into_iter().enumerate().map(|(i, n)| (i + 1, n)) {
        print!("{} 1", i);
        for connection in n.into_iter().map(|i| i + 1) {
            print!(" {connection}");
        }
        println!(" 0");
    }
}
