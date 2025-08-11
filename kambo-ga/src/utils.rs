use kambo_core::problem::Problem;
use kambo_core::solution::Solution;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(not(feature = "parallel"))]
pub(crate) fn evaluate_population<P: Problem>(problem: &P, population: &mut [P::Solution]) {
    for solution in population {
        problem.evaluate(solution);
    }
}

#[cfg(feature = "parallel")]
pub(crate) fn evaluate_population<P: Problem>(problem: &P, population: &mut [P::Solution])
where
    P: Problem + Sync,
    P::Solution: Send,
{
    population.par_iter_mut().for_each(|solution| {
        problem.evaluate(solution);
    });
}
