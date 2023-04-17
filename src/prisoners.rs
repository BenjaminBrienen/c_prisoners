#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Prisoner
{
	pub id:      usize,
	pub freedom: Freedom,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Freedom
{
	Free,
	NotFree,
}

pub fn generate_prisoners(prisoners_count: usize) -> Vec<Prisoner>
{
	(0..prisoners_count)
		.map(|id| Prisoner { id, freedom: Freedom::NotFree })
		.collect()
}
