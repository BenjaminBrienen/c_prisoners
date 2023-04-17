use rand::{
	seq::SliceRandom,
	thread_rng,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Box
{
	pub id:    usize,
	pub value: usize,
}

pub fn generate_boxes(boxes_count: usize) -> Vec<Box>
{
	let mut boxes_inner: Vec<_> = (0..boxes_count).collect();
	boxes_inner.shuffle(&mut thread_rng());
	(0..boxes_count)
		.zip(boxes_inner)
		.map(|(id, value)| Box { id, value })
		.collect()
}
