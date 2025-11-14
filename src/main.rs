use linfa::Dataset;
use linfa::clustering::{KMeans, KMeansHyperParams};
use ndarray::Array2;
use petgraph::prelude::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
implementation of a subnetwork clustering of the bacterial metagenomics graphs.
you can provide the network data with the nodes and then it will implement the
graph kmeans clustering alogrithm on the same and will give you the defined clusters.
converting all the graph algorithms to RUST and applying them to the genomics data.
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Graphadd {
    graphedges: (usize, usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 0usize {
        println!("The arguments cant be empty");
    }
    if args.len() > 1 {
        let value = args[1];
        let kvalue: usize = args[2].parse::<usize>().unwrap();
        let fileopen = File::open(args[1]).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut graphnode: Vec<Graphadd> = Vec::new();
        for i in fileread.lines() {
            let linesplit = i.expect("file not present");
            let linevec = linesplit.split(",").collect::<Vec<_>>();
            let (node1, node2): (usize, usize) = (
                linevec[0].parse::<usize>().unwrap(),
                linevec[1].parse::<usize>().unwrap(),
            );
            graphnode.push(Graphadd {
                graphedges: (node1, node2),
            });
        }
        let finalvec = graphnode.into_iter().collect::<Vec<_>>();
        let mut graphadd: Graph<(), (), Undirected> = Graph::new_undirected();
        graphadd.extend_with_edges(&finalvec);
        let nodescount = graphadd.node_count();
        let mut fea = Array2::zeros((nodescount as usize, 1));
        for (i, node) in graphadd.node_indices().enumerate() {
            fea[[i, 0]] = graphadd.neighbors_undirected(node).count() as f64;
        }
        let dataset = Dataset::from(fea);
        let kclusters = kvalue;
        let hyperparams = KMeansHyperParams::default().n_clusters(kclusters);
        let kmeans = KMeans::fit(hyperparams, &dataset).unwrap();
        let clusters = kmeans.predict(&dataset);
        println!("Node clusters: {:?}", clusters.targets);
    }
}
