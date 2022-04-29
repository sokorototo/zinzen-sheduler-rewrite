use super::repetition::Repetition;
use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson, Default)]
pub struct Task {
	description: String,
	duration: usize,
	repetition: Repetition,
}
