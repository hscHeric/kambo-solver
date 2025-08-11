use kambo_core::Solution;
use rand::Rng;

pub trait MutationOperator<S: Solution> {
    fn mutate<R: Rng>(&self, rng: &mut R, solution: &mut S);
}
