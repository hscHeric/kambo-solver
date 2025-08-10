use crate::{
    problem::Problem, solution::Solution, state::AlgorithmState, termination::TerminationCriteria,
};

pub trait Metaheuristic<P: Problem> {
    fn initialize(&self, problem: &P) -> AlgorithmState<P::Solution>;
    fn step(&self, problem: &P, state: &mut AlgorithmState<P::Solution>) -> Option<P::Solution>;
}

pub struct Solver<'a, P, M, T>
where
    P: Problem,
    M: Metaheuristic<P>,
    T: TerminationCriteria<P::Solution>,
{
    problem: &'a P,
    metaheuristic: &'a M,
    termination: &'a T,
}

impl<'a, P, M, T> Solver<'a, P, M, T>
where
    P: Problem,
    M: Metaheuristic<P>,
    T: TerminationCriteria<P::Solution>,
{
    pub fn new(problem: &'a P, metaheuristic: &'a M, termination: &'a T) -> Self {
        Self {
            problem,
            metaheuristic,
            termination,
        }
    }

    /// Executa o processo de otimização e retorna a melhor solução encontrada.
    pub fn run(&self) -> P::Solution {
        let mut state = self.metaheuristic.initialize(self.problem);

        while !self.termination.should_terminate(&state) {
            if let Some(step_best) = self.metaheuristic.step(self.problem, &mut state) {
                if P::GOAL.is_better(step_best.fitness(), state.best_solution.fitness()) {
                    state.best_solution = step_best;
                }
            }
            state.iteration_count += 1;
        }
        state.best_solution
    }
}
