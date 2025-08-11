use kambo_core::Problem;

pub trait InitialSolutionHeuristic<P: Problem> {
    fn generate(&self, problem: &P) -> P::Solution;
}

pub struct HybridInitializer<'a, P: Problem> {
    pub heuristics: Vec<(Box<dyn InitialSolutionHeuristic<P> + 'a>, f64)>,
}

impl<P: Problem> HybridInitializer<'_, P> {
    pub fn initialize_population(&self, problem: &P, pop_size: usize) -> Vec<P::Solution> {
        let mut population = Vec::with_capacity(pop_size);
        let mut remaining = pop_size;

        for (heuristic, percentage) in &self.heuristics {
            let count = (pop_size as f64 * percentage).round() as usize;
            for _ in 0..count {
                if remaining > 0 {
                    population.push(heuristic.generate(problem));
                    remaining -= 1;
                }
            }
        }
        while remaining > 0 {
            population.push(self.heuristics[0].0.generate(problem));
            remaining -= 1;
        }
        population
    }
}
