pub mod initializer;
pub mod operators;
mod utils;

use crate::initializer::HybridInitializer;
use crate::operators::{
    crossover::CrossoverOperator,
    mutation::MutationOperator,
    repair::{NoOpRepair, RepairOperator},
    selection::SelectionOperator,
};
use crate::utils::evaluate_population;
use kambo_core::Solution;
use kambo_core::{problem::Problem, solver::Metaheuristic, state::AlgorithmState};
use rand::{Rng, rngs::ThreadRng};
use std::cell::RefCell;
use std::marker::PhantomData;

pub struct GeneticAlgorithm<'a, P: Problem, Sel, Cross, Mut, Rep = NoOpRepair>
where
    P::Solution: 'a,
    Sel: SelectionOperator<P::Solution>,
    Cross: CrossoverOperator<P::Solution>,
    Mut: MutationOperator<P::Solution>,
    Rep: RepairOperator<P::Solution>,
{
    pop_size: usize,
    crossover_rate: f64,
    mutation_rate: f64,
    initializer: HybridInitializer<'a, P>,
    selection: &'a Sel,
    crossover: &'a Cross,
    mutation: &'a Mut,
    repair: Option<&'a Rep>,
    population: RefCell<Vec<P::Solution>>,
    rng: ThreadRng,
    _problem: PhantomData<P>,
}

impl<'a, P, Sel, Cross, Mut, Rep> GeneticAlgorithm<'a, P, Sel, Cross, Mut, Rep>
where
    P: Problem,
    P::Solution: 'a,
    Sel: SelectionOperator<P::Solution>,
    Cross: CrossoverOperator<P::Solution>,
    Mut: MutationOperator<P::Solution>,
    Rep: RepairOperator<P::Solution>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pop_size: usize,
        crossover_rate: f64,
        mutation_rate: f64,
        initializer: HybridInitializer<'a, P>,
        selection: &'a Sel,
        crossover: &'a Cross,
        mutation: &'a Mut,
        repair: Option<&'a Rep>,
    ) -> Self {
        Self {
            pop_size,
            crossover_rate,
            mutation_rate,
            initializer,
            selection,
            crossover,
            mutation,
            repair,
            population: RefCell::new(Vec::with_capacity(pop_size)),
            rng: rand::rng(),
            _problem: PhantomData,
        }
    }
}

impl<'a, P, Sel, Cross, Mut, Rep> Metaheuristic<P> for GeneticAlgorithm<'a, P, Sel, Cross, Mut, Rep>
where
    P: Problem + Sync,
    P::Solution: 'a + Send + Sync,
    Sel: SelectionOperator<P::Solution> + Sync,
    Cross: CrossoverOperator<P::Solution> + Sync,
    Mut: MutationOperator<P::Solution> + Sync,
    Rep: RepairOperator<P::Solution> + Sync,
{
    fn initialize(&self, problem: &P) -> AlgorithmState<P::Solution> {
        let mut initial_pop = self
            .initializer
            .initialize_population(problem, self.pop_size);

        evaluate_population(problem, &mut initial_pop);

        let best_initial = initial_pop
            .iter()
            .min_by(|a, b| {
                if P::GOAL.is_better(a.fitness(), b.fitness()) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .expect("Initial population cannot be empty.")
            .clone();

        *self.population.borrow_mut() = initial_pop;

        AlgorithmState {
            iteration_count: 0,
            evaluations_count: self.pop_size,
            start_time: std::time::Instant::now(),
            best_solution: best_initial,
        }
    }

    fn step(&self, problem: &P, state: &mut AlgorithmState<P::Solution>) -> Option<P::Solution> {
        let mut rng = self.rng.clone();

        // Criamos um novo escopo para o empréstimo imutável
        let mut new_population = {
            let old_population = self.population.borrow(); // Empréstimo começa aqui
            let mut children = Vec::with_capacity(self.pop_size);

            for _ in 0..(self.pop_size / 2) {
                let (parent1, parent2) = self.selection.select(&mut rng, &old_population, P::GOAL);

                let (mut child1, mut child2) = if rng.random_bool(self.crossover_rate) {
                    self.crossover.crossover(&mut rng, parent1, parent2)
                } else {
                    (parent1.clone(), parent2.clone())
                };

                if rng.random_bool(self.mutation_rate) {
                    self.mutation.mutate(&mut rng, &mut child1);
                }
                if rng.random_bool(self.mutation_rate) {
                    self.mutation.mutate(&mut rng, &mut child2);
                }

                if let Some(repair_op) = self.repair {
                    repair_op.repair(&mut child1);
                    repair_op.repair(&mut child2);
                }

                children.push(child1);
                children.push(child2);
            }
            children
        };

        evaluate_population(problem, &mut new_population);
        state.evaluations_count += new_population.len();

        let generation_best = new_population
            .iter()
            .min_by(|a, b| {
                if P::GOAL.is_better(a.fitness(), b.fitness()) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .cloned();

        *self.population.borrow_mut() = new_population;

        generation_best
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        initializer::{HybridInitializer, InitialSolutionHeuristic},
        operators::{
            crossover::{AsVectorSolution, OnePoint},
            mutation::MutationOperator,
            selection::TournamentSelection,
        },
    };
    use kambo_core::{
        problem::OptimizationGoal, solution::Solution, solver::Solver, termination::ByIterations,
    };
    use rand::Rng;

    #[derive(Debug, Clone)]
    struct VecSolution {
        genes: Vec<f64>,
        fitness: f64,
    }

    impl Solution for VecSolution {
        fn fitness(&self) -> f64 {
            self.fitness
        }
        fn set_fitness(&mut self, fitness: f64) {
            self.fitness = fitness;
        }
    }

    impl AsVectorSolution for VecSolution {
        type Gene = f64;
        fn genes(&self) -> &[Self::Gene] {
            &self.genes
        }
        fn genes_mut(&mut self) -> &mut [Self::Gene] {
            &mut self.genes
        }
    }

    struct DummyProblem;

    impl Problem for DummyProblem {
        type Solution = VecSolution;
        const GOAL: OptimizationGoal = OptimizationGoal::Minimize;

        fn evaluate(&self, solution: &mut Self::Solution) {
            let fitness = solution.genes.iter().sum();
            solution.set_fitness(fitness);
        }
    }

    struct RandomInitializer;
    impl InitialSolutionHeuristic<DummyProblem> for RandomInitializer {
        fn generate(&self, _problem: &DummyProblem) -> VecSolution {
            let mut rng = rand::rng();
            let genes: Vec<f64> = (0..10).map(|_| rng.random_range(0.0..100.0)).collect();
            VecSolution {
                genes,
                fitness: f64::INFINITY,
            }
        }
    }

    struct DummyMutation;
    impl MutationOperator<VecSolution> for DummyMutation {
        fn mutate<R: Rng>(&self, rng: &mut R, solution: &mut VecSolution) {
            if !solution.genes.is_empty() {
                let index = rng.random_range(0..solution.genes.len());
                solution.genes[index] += rng.random_range(-1.0..1.0);
            }
        }
    }

    #[test]
    fn test_ga_solver_runs_successfully() {
        // 1. Setup dummy components
        let problem = DummyProblem;
        let initializer = RandomInitializer;
        let selection = TournamentSelection::new(2);
        let crossover = OnePoint;
        let mutation = DummyMutation;
        let repair = NoOpRepair;
        // 2. Configure the initializer
        let hybrid_initializer = HybridInitializer::new(vec![(Box::new(initializer), 1.0)]);

        // 3. Instantiate the Genetic Algorithm
        let ga = GeneticAlgorithm::new(
            100, // pop_size
            0.8, // crossover_rate
            0.1, // mutation_rate
            hybrid_initializer,
            &selection,
            &crossover,
            &mutation,
            Some(&repair), // Explicitly passing None for the repair operator
        );

        let termination = ByIterations {
            max_iterations: 100,
        };
        let solver = Solver::new(&problem, &ga, &termination);

        let best_solution = solver.run();

        assert!(best_solution.fitness.is_finite());
        println!(
            "Test completed. Best fitness found: {}",
            best_solution.fitness()
        );
    }
}
