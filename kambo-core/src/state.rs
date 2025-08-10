use std::time::Instant;

use crate::solution::Solution;

pub struct AlgorithmState<S: Solution> {
    pub iteration_count: usize,
    pub evaluations_count: usize,
    pub start_time: Instant,
    pub best_solution: S,
}
