use nanoserde::{DeJson, SerJson};

/// An internal ID that is auto-incremented for each goal declared
static mut AUTO_INCREMENTING_ID: usize = 0;

/// A [Goal] is what one wants to do, it is used in conjunction with a [Timeline] to generate a [Schedule]
#[derive(DeJson, SerJson, Debug)]
#[non_exhaustive]
pub struct Goal {
	pub id: usize,
	pub description: String,
	pub fixed_time: Option<usize>,
	pub duration: usize,
	pub repetition: Repetition,
	pub priority: usize,
}

impl Default for Goal {
	fn default() -> Self {
		let new_id = unsafe {
			AUTO_INCREMENTING_ID += 1;
			AUTO_INCREMENTING_ID
		};

		Self {
			id: new_id,
			description: Default::default(),
			fixed_time: Default::default(),
			duration: Default::default(),
			repetition: Default::default(),
			priority: Default::default(),
		}
	}
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
