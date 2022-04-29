use nanoserde::{DeJson, SerJson};

/// A [Goal] is what one wants to do, it is used in conjunction with a [Timeline] to generate a [Schedule]
#[derive(DeJson, SerJson, Default, Debug)]
pub struct Goal {
	pub description: String,
	pub fixed_time: Option<usize>,
	pub duration: usize,
	pub repetition: Repetition,
	pub priority: usize,
}

#[derive(DeJson, SerJson, Debug)]
pub enum Repetition {
	Exact(usize),
	Once,
	Daily,
	Weekly,
	Monthly,
	Annually,
}

impl Default for Repetition {
	fn default() -> Repetition {
		Repetition::Once
	}
}
