use crate::goal::Goal;

/// A [Task] is an item a user is expected to accomplish, it is defined in a user's schedule, which itself is just a contiguous list of tasks
pub struct Task<'a> {
	pub(crate) goal: &'a Goal,
	pub(crate) completed: bool,
}

impl<'a> Task<'a> {
	pub fn serialize_json(&self) -> String {
		format!("{{\"goal_id\": {}, \"completed\": {}}}", self.goal.id, self.completed)
	}
}

impl<'a> Task<'a> {
	pub fn new(goal: &Goal) -> Task {
		Task { goal, completed: false }
	}
}
