use std::time::Duration;

use crate::{solution::Solution, state::AlgorithmState};

pub trait TerminationCriteria<S: Solution> {
    fn should_terminate(&self, state: &AlgorithmState<S>) -> bool;
}

pub struct ByDuration {
    pub time_limit: Duration,
}

impl<S: Solution> TerminationCriteria<S> for ByDuration {
    fn should_terminate(&self, state: &AlgorithmState<S>) -> bool {
        state.start_time.elapsed() >= self.time_limit
    }
}

pub struct ByIterations {
    pub max_iterations: usize,
}

impl<S: Solution> TerminationCriteria<S> for ByIterations {
    fn should_terminate(&self, state: &AlgorithmState<S>) -> bool {
        state.iteration_count >= self.max_iterations
    }
}
