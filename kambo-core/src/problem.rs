use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationGoal {
    Minimize,
    Maximize,
}

impl OptimizationGoal {
    #[inline]
    pub fn is_better(&self, new_fitness: f64, current_fitness: f64) -> bool {
        match self {
            //valores menores são melhores.
            OptimizationGoal::Minimize => new_fitness < current_fitness,
            //valores maiores são melhores.
            OptimizationGoal::Maximize => new_fitness > current_fitness,
        }
    }
}

pub trait Problem {
    type Solution: Solution;
    const GOAL: OptimizationGoal;
    fn evaluate(&self, solution: &mut Self::Solution);
}
