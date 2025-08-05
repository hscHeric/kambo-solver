use std::time::Instant;

use crate::{problem::Problem, solution::Solution, termination::TerminationCriteria};

pub struct IterationState<S: Solution> {
    pub current_generation: u32,
    pub start_time: Instant,
    pub best_solution_so_far: Option<S>,
    pub no_improvement_streak: u32,
}

impl<S: Solution> IterationState<S> {
    pub fn new() -> Self {
        Self {
            current_generation: 0,
            start_time: Instant::now(),
            best_solution_so_far: None,
            no_improvement_streak: 0,
        }
    }

    pub fn should_terminate(&self, criteria: &TerminationCriteria) -> bool {
        match criteria {
            TerminationCriteria::MaxGenerations(max) => self.current_generation >= *max,
            TerminationCriteria::MaxDuration(max) => self.start_time.elapsed() >= *max,
            TerminationCriteria::NoImprovementFor(max) => self.no_improvement_streak >= *max,
        }
    }
}

impl<S: Solution> Default for IterationState<S> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Algorithm<S: Solution> {
    fn solve<P: Problem<S>>(&self, problem: &P, termination: &TerminationCriteria) -> Option<S>;
}
