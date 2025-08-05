use crate::solution::Solution;

pub trait Problem<S: Solution>: Send + Sync {
    fn evaluate(&self, solution: &mut S);
    fn initial_solution(&self) -> S;
}
