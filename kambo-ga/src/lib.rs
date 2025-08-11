pub mod initializer;
pub mod operators;
mod utils;

use crate::initializer::HybridInitializer;
use crate::operators::{
    crossover::CrossoverOperator,
    mutation::MutationOperator,
    // Corrigido: Importa NoOpRepair para usá-lo como padrão.
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
        let mut new_population = Vec::with_capacity(self.pop_size);

        let old_population = self.population.borrow();

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

            new_population.push(child1);
            new_population.push(child2);
        }

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

