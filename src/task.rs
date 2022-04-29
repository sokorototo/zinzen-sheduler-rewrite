use crate::goal::Goal;

/// A [Task] is an item a user is expected to accomplish, it is defined in a user's schedule, which itself is just a contiguous list of tasks
#[derive(Debug)]
pub struct Task<'a> {
	pub(crate) goal: &'a Goal,
	pub(crate) completed: bool,
}

impl<'a> Task<'a> {
	pub fn new(goal: &Goal) -> Task {
		Task { goal, completed: false }
	}
}
