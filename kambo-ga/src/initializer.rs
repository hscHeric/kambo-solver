use kambo_core::Problem;

pub trait InitialSolutionHeuristic<P: Problem> {
    fn generate(&self, problem: &P) -> P::Solution;

    fn is_deterministic(&self) -> bool {
        false
    }
}

pub struct HybridInitializer<'a, P: Problem> {
    pub heuristics: Vec<(Box<dyn InitialSolutionHeuristic<P> + 'a + Send + Sync>, f64)>,
}

impl<'a, P: Problem> HybridInitializer<'a, P> {
    #[must_use]
    pub fn new(
        heuristics: Vec<(Box<dyn InitialSolutionHeuristic<P> + 'a + Send + Sync>, f64)>,
    ) -> Self {
        Self { heuristics }
    }

    /// /// Generates a population based on the configured heuristics.
    ///
    /// # Panics
    ///
    /// Panics if the list of heuristics is empty, as there would be no way to generate
    /// the initial solutions. This check occurs when the function tries to fill
    /// any remaining spots in the population after the main loop.
    pub fn initialize_population(&self, problem: &P, pop_size: usize) -> Vec<P::Solution> {
        let mut population = Vec::with_capacity(pop_size);

        for (heuristic, percentage) in &self.heuristics {
            #[allow(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                clippy::cast_precision_loss
            )]
            let count = (pop_size as f64 * *percentage).round() as usize;
            if count == 0 {
                continue;
            }

            if heuristic.is_deterministic() {
                let base_solution = heuristic.generate(problem);
                for _ in 0..count {
                    if population.len() < pop_size {
                        population.push(base_solution.clone());
                    }
                }
            } else {
                for _ in 0..count {
                    if population.len() < pop_size {
                        population.push(heuristic.generate(problem));
                    }
                }
            }
        }

        while population.len() < pop_size {
            if let Some((heuristic, _)) = self.heuristics.first() {
                population.push(heuristic.generate(problem));
            } else {
                panic!("No initialization heuristics were provided.");
            }
        }

        population
    }
}
