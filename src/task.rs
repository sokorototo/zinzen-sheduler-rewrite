use crate::goal::Goal;

/// A [Task] is an item a user is expected to accomplish, it is defined in a user's schedule, which itself is just a contiguous list of tasks
#[derive(Debug)]
pub struct Task<'a> {
	pub(crate) goal: &'a Goal,
	pub(crate) completed: bool,
}

impl<'a> Task<'a> {
	pub fn to_bytes(&self) -> [usize; 2] {
		let mut buf = [0; 2];

		buf[0] = self.goal.id;
		buf[1] = self.completed.into();

		buf
	}
}

impl<'a> Task<'a> {
	pub fn new(goal: &Goal) -> Task {
		Task { goal, completed: false }
	}
}
