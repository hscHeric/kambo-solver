use kambo_core::{Problem, Solution};
use rand::seq::IndexedRandom;

pub trait SelectionOperator<S: Solution> {
    fn select<'a, P: Problem<Solution = S>>(
        &self,
        problem: &'a P,
        population: &'a [S],
    ) -> Vec<usize>;
}

pub struct TournamentSelection {
    pub tournament_size: usize,
}

impl<S: Solution> SelectionOperator<S> for TournamentSelection {
    fn select<'a, P: Problem<Solution = S>>(
        &self,
        problem: &'a P,
        population: &'a [S],
    ) -> Vec<usize> {
        let mut rng = rand::rng();
        let mut selected_indices = Vec::with_capacity(population.len());

        for _ in 0..population.len() {
            // Escolhe N competidores aleatórios para o torneio
            let competitors_indices: Vec<usize> = (0..population.len())
                .collect::<Vec<_>>()
                .choose_multiple(&mut rng, self.tournament_size)
                .copied()
                .collect();

            let winner_index = competitors_indices
                .into_iter()
                .min_by(|&a, &b| {
                    let fitness_a = population[a].fitness();
                    let fitness_b = population[b].fitness();
                    if P::GOAL.is_better(fitness_a, fitness_b) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                })
                .unwrap();

            selected_indices.push(winner_index);
        }
        selected_indices
    }
}
