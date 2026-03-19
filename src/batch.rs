use bitvec::prelude::{BitVec};

pub trait BatchEval: Sized {
    fn generate_inputs(arity: usize) -> Vec<Self>;
    fn measure_fitness(output: &[Self], target: &[Self]) -> usize;
}


impl BatchEval for usize {
    fn generate_inputs(arity : usize) -> Vec<usize> {
	let mut inputs = Vec::new();
	if arity == 0 {
	    return inputs
	}
	for column in 0..arity {
	    let mut curr : usize = 0;
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


#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
	#[test]
	// Note: for u64 we can only handle up to arity 6 which corresponds to
	// a bitvector of size 2^6 = 64
	fn generate_inputs_usize(arity in 1usize..6) {
	let inputs = usize::generate_inputs(arity);
	for i in 0..(1 << arity) {
	    // The generated inputs are the columns of a complete truth table
	    // the corresponding rows would just be the binary representation
	    // of the numbers from 0 to 2^arity - 1

	    // It should be the case that if we get the ith bit from each column
	    // and then bitshift it appropriately, we get back the number of the column

	    let row: usize = inputs.iter().enumerate() // label each column by its index
		.map(|(k, col)| ((col >> i) & 1usize) << k)
	    // extract the ith row bit from the kth column and then shift to its placeholder
		.sum();
	    // sum up all the bits (same as bitwise or)
            prop_assert_eq!(row, i);
	}

	}
    }

}
