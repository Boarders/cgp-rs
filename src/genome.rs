use crate::function_basis::FunctionSet;
use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeIdx {
    idx: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node<b> {
    left: NodeIdx,
    right: NodeIdx,
    func_idx: b,
}

pub struct Genome<a> {
    nodes: Vec<Node<a>>,
    output_nodes: Vec<NodeIdx>
}

pub fn run_genome<T, b>(genome: &Genome<b>, inputs: &[T]) -> Vec<T>
where
    b: FunctionSet<T>,
    T: Copy,
{
    let mut values: Vec<T> = Vec::from(inputs);
    for node in &genome.nodes {
	let lhs = values[node.left.idx as usize];
	let rhs = values[node.right.idx as usize];
	let val = node.func_idx.apply(lhs, rhs);
	values.push(val);
    }
    let mut output: Vec<T> = Vec::new();
    for output_idx in &genome.output_nodes {
	output.push(values[output_idx.idx as usize])
    }
    output
}
