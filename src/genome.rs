use crate::function_basis::FunctionSet;
use crate::batch::BatchEval;
use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeIdx {
    idx: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node<B> {
    left: NodeIdx,
    right: NodeIdx,
    function_index: B,
}

// The nodes field corresponds to a DAG that computes a given generalized circuit
// which we can think of as follows:
//
//
// A ─────┬───────────────┐
// │      │               │
// │      └──► N0 ◄───────┘
// │           │
// │  B ──┬────┼──────────┐
// │      │    ▼   ┌──────┘
// │      │    N1 ◄┘
// │      │     └────────────┐
// │      ├───────────────┐  │
// │      │               │  │
// │      └──► N2 ◄───────┘  │
// │           │             │
// └───────────┼──────────┐  │
//             ▼   ┌──────┘  │
//             N3 ◄┘         │
//            │ ┌────────────┘
//            │ │
//            ▼ ▼
//            N4 [OUT]
//
// This is a NAND network computing XOR where each node has only NAND as the function index:
//   N0 = NAND(A, A) = !A
//   N1 = NAND(N0, B) = !(!AB)
//   N2 = NAND(B, B) = !B
//   N3 = NAND(N2, A) = !(A!B)
//   N4 = NAND(N1, N3) = !(!(!AB) AND !(A!B))
//                     = !!(!AB) + !!(A!B)
//                     = !AB + A!B
//                     = A XOR B

pub struct Genome<B> {
    nodes: Vec<Node<B>>,
    output_nodes: Vec<NodeIdx>
}

// Here inputs is a slice which we can think of as an environment
// mapping variables which we identify as indices to values of our
// input type for the kind of circuit we are computing
pub fn run_genome<T, B>(genome: &Genome<B>, inputs: &[T]) -> Vec<T>
where
    B: FunctionSet<T>,
    T: Copy,
{
    let mut values: Vec<T> = Vec::from(inputs);
    for node in &genome.nodes {
	let lhs = values[node.left.idx as usize];
	let rhs = values[node.right.idx as usize];
	let val = node.function_index.apply(lhs, rhs);
	values.push(val);
    }
    let mut output: Vec<T> = Vec::new();
    for output_idx in &genome.output_nodes {
	output.push(values[output_idx.idx as usize])
    }
    output
}

pub fn evolve<T, B>(arity: usize, genome: &Genome<B>) -> Genome<B>
where
    B: FunctionSet<T>,
    T: BatchEval + Copy
{
    let batch_inputs = T::generate_inputs(arity);
    todo!()

}
