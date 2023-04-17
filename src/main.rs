use {
	c_prisoners::*,
	std::io::*,
};

fn main() -> Result<()>
{
	// Setup.
	let mut input = stdin().lock();
	let mut output = stdout().lock();
	let prisoners_count = get_input("How many prisoners are there? (defaults to 100): ", &mut input, &mut output, 100)?;
	let tests_count = get_input("How many tests to complete? (defaults to 100): ", &mut input, &mut output, 100)?;

	// Run tests.
	let random_successes = (0..tests_count)
		.filter(|_| simulate_random_choice_strategy_successful(prisoners_count))
		.count();
	let optimal_successes = (0..tests_count)
		.filter(|_| simulate_optimal_strategy_successful(prisoners_count))
		.count();

	// Output.
	println!("Random method successes: {}", random_successes);
	println!("Optimal method successes: {}", optimal_successes);
	Ok(())
}

fn get_input<TBufRead: BufRead, TWrite: Write>(
	prompt: &str,
	input: &mut TBufRead,
	output: &mut TWrite,
	default: usize,
) -> Result<usize>
{
	let mut prisoners_size_input = String::new();
	writeln!(output, "{prompt}")?;
	_ = input
		.read_line(&mut prisoners_size_input)
		.expect("Failure to read input");
	Ok(prisoners_size_input.trim().parse().unwrap_or(default))
}
