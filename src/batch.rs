use bitvec::prelude::{BitVec};

pub trait BatchEval: Sized {
    fn generate_inputs(arity: usize) -> Vec<Self>;
    fn measure_fitness(output: &[Self], target: &[Self]) -> usize;
}


impl BatchEval for u64 {
    fn generate_inputs(arity : usize) -> Vec<u64> {
	let mut inputs = Vec::new();
	if arity == 0 {
	    return inputs
	}
	for column in 0..arity {
	    let mut curr : u64 = 0;
	    for i in 0..(1 << arity) {
		if (i >> column) & 1 == 1 {
		    curr = curr | (1 << i);
		}
	    }
	    inputs.push(curr);
	}
	inputs
    }

    fn measure_fitness(output: &[Self], target: &[Self]) -> usize {
	output.iter().zip(target.iter()).map(|(o, t)| (o ^ t).count_ones()).sum::<u32>() as usize
    }
}


impl BatchEval for BitVec {
    fn generate_inputs(arity : usize) -> Vec<BitVec> {
	let mut inputs = Vec::new();
	if arity == 0 {
	    return inputs
	}
	for column in 0..arity {
	    let mut curr : BitVec = BitVec::new();
	    for i in 0..(1 << arity) {
		if (i >> column) & 1 == 1 {
		    curr.push(true);
		}
		else {
		    curr.push(false)
		}
	    }
	    inputs.push(curr);
	}
	inputs
    }

    fn measure_fitness(output: &[Self], target: &[Self]) -> usize {
	output.iter().zip(target.iter()).map(|(o, t)| (o.clone() ^ t.clone()).count_ones()).sum()
    }
}
