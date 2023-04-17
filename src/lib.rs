use {
	boxes::*,
	prisoners::*,
	rand::prelude::*,
};

pub mod boxes;
pub mod prisoners;

pub fn simulate_random_choice_strategy_successful(prisoners_count: usize) -> bool
{
	let mut prisoners = generate_prisoners(prisoners_count);
	let boxes = generate_boxes(prisoners_count);
	let open_boxes_limit = prisoners_count / 2;
	for prisoner in &mut prisoners
	{
		// Free prisoner if they randomly chose the box with their own number
		if boxes
			.choose_multiple(&mut thread_rng(), open_boxes_limit)
			.any(|r#box| r#box.id == prisoner.id)
		{
			prisoner.freedom = Freedom::Free;
		}
	}
	!prisoners
		.iter()
		.any(|prisoner| prisoner.freedom == Freedom::NotFree)
}

pub fn simulate_optimal_strategy_successful(prisoners_count: usize) -> bool
{
	let mut prisoners = generate_prisoners(prisoners_count);
	let boxes = generate_boxes(prisoners_count);
	for prisoner in &mut prisoners
	{
		prisoner.freedom = find_box_with_value_matching_prisoner_id(prisoner, prisoners_count, &boxes);
	}
	!prisoners
		.iter()
		.any(|prisoner| prisoner.freedom == Freedom::NotFree)
}

fn find_box_with_value_matching_prisoner_id(
	prisoner: &Prisoner,
	prisoners_count: usize,
	boxes: &[Box],
) -> Freedom
{
	let open_boxes_limit = prisoners_count / 2;
	let mut current_box = &boxes[prisoner.id];
	let mut opened_boxes = 1;
	loop
	{
		// If they found their number, free them.
		if current_box.value == prisoner.id
		{
			return Freedom::Free
		}
		// They open the next referenced box.
		current_box = &boxes[current_box.value];
		// If they've reached the limit, they don't get freed.
		if opened_boxes == open_boxes_limit
		{
			return Freedom::NotFree
		}
		opened_boxes += 1;
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	const TESTS: usize = 9999;
	const PRISONERS_COUNT: usize = 100;

	#[test]
	pub fn optimal_better_than_random()
	{
		let random_successes = (0..TESTS)
			.filter(|_| simulate_random_choice_strategy_successful(PRISONERS_COUNT))
			.count();
		let optimal_successes = (0..TESTS)
			.filter(|_| simulate_optimal_strategy_successful(PRISONERS_COUNT))
			.count();
		dbg!(random_successes);
		dbg!(optimal_successes);
		assert!(random_successes < optimal_successes);
	}

	#[test]
	pub fn random_strategy_bad()
	{
		let random_successes = (0..TESTS)
			.filter(|_| simulate_random_choice_strategy_successful(PRISONERS_COUNT))
			.count();
		// less than 1%
		dbg!(random_successes);
		assert!(random_successes < TESTS / 100);
	}

	#[test]
	pub fn optimal_strategy_between_25_and_33_percent()
	{
		let optimal_successes = (0..TESTS)
			.filter(|_| simulate_optimal_strategy_successful(PRISONERS_COUNT))
			.count();
		dbg!(optimal_successes);
		assert!(optimal_successes > TESTS / 4);
		assert!(optimal_successes < TESTS / 3);
	}
}
