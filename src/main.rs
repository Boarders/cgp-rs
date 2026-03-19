use cartesian_genetic_programming::batch::BatchEval;

fn main() {
    let inputs : Vec<u64> = BatchEval::generate_inputs(3);
    for input in inputs {
	println!("{:#b}", input);
    }
}
