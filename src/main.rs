use cartesian_genetic_programming::batch::BatchEval;

fn main() {
    let inputs : Vec<usize> = BatchEval::generate_inputs(3);
    for input in inputs {
	println!("{:#b}", input);
    }
}
