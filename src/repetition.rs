use nanoserde::{DeJson, SerJson};

#[derive(DeJson, SerJson)]
pub enum Repetition {
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
