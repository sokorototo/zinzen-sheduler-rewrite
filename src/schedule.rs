use crate::task::Task;

pub struct Schedule<'a> {
	tasks: Vec<Task<'a>>,
}
