pub trait BatchEval: Sized {
    fn generate_inputs(arity: usize) -> Vec<Self>;
    fn measure_fitness(output: &[Self], target: &[Self]) -> u32;
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

    fn measure_fitness(output: &[Self], target: &[Self]) -> u32 {
	output.iter().zip(target.iter()).map(|(o, t)| (o ^ t).count_ones()).sum()
    }
}
