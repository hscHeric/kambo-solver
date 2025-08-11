use kambo_core::{OptimizationGoal, Solution};
use rand::{Rng, seq::IndexedRandom};

pub trait SelectionOperator<S: Solution> {
    fn select<'a, R: Rng>(
        &self,
        rng: &mut R,
        population: &'a [S],
        goal: OptimizationGoal,
    ) -> (&'a S, &'a S);
}

pub struct TournamentSelection {
    size: usize,
}

impl TournamentSelection {
    /// Creates a new Tournament Selection operator.
    ///
    /// # Panics
    /// Panics if `tournament_size` is 0.
    #[must_use]
    pub fn new(tournament_size: usize) -> Self {
        assert!(
            tournament_size > 0,
            "Tournament size must be greater than 0."
        );
        Self {
            size: tournament_size,
        }
    }

    fn run_tournament<'a, R: Rng, S: Solution>(
        &self,
        rng: &mut R,
        population: &'a [S],
        goal: OptimizationGoal,
    ) -> &'a S {
        let competitors = population.choose_multiple(rng, self.size);

        competitors
            .min_by(|a, b| {
                if goal.is_better(a.fitness(), b.fitness()) {
                    std::cmp::Ordering::Less
                } else if goal.is_better(b.fitness(), a.fitness()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .expect("Tournament cannot be run with an empty population or tournament size of 0.")
    }
}

impl<S: Solution> SelectionOperator<S> for TournamentSelection {
    fn select<'a, R: Rng>(
        &self,
        rng: &mut R,
        population: &'a [S],
        goal: OptimizationGoal,
    ) -> (&'a S, &'a S) {
        let parent1 = self.run_tournament(rng, population, goal);
        let parent2 = self.run_tournament(rng, population, goal);
        (parent1, parent2)
    }
}
