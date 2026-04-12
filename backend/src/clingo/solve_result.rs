//! This module provides the Clingo solve result representation.

use bitflags::bitflags;

use super::bindings::{
    clingo_solve_result_e_clingo_solve_result_exhausted,
    clingo_solve_result_e_clingo_solve_result_interrupted,
    clingo_solve_result_e_clingo_solve_result_satisfiable,
    clingo_solve_result_e_clingo_solve_result_unsatisfiable,
};

bitflags! {
    #[derive(Debug)]
    /// The result of a solve operation.
    pub struct SolveResult: u32 {
        /// The solve operation found a satisfiable solution.
        const SATISFIABLE = clingo_solve_result_e_clingo_solve_result_satisfiable as u32;
        /// The solve operation found the problem to be unsatisfiable.
        const UNSATISFIABLE =
            clingo_solve_result_e_clingo_solve_result_unsatisfiable as u32;
        /// The solve operation exhausted the search space.
        const EXHAUSTED = clingo_solve_result_e_clingo_solve_result_exhausted as u32;
        /// The solve operation was interrupted before completion.
        const INTERRUPTED = clingo_solve_result_e_clingo_solve_result_interrupted as u32;
    }
}
