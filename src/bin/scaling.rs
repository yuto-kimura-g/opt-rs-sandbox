// https://github.com/rust-or/good_lp/blob/main/tests/solver_scaling.rs

// use float_eq::assert_float_eq;
use good_lp::{
    constraint, default_solver,
    solvers::coin_cbc::{CoinCbcProblem, CoinCbcSolution},
    variable, variables, Expression, ProblemVariables, Solution, SolverModel,
};

const BIG_NUM: usize = 1_000_000; // <- Set this higher to test how good_lp and the solvers scale

fn main() {
    let mut vars: ProblemVariables = variables!();
    let min = -((BIG_NUM / 2) as f64);
    let max = (BIG_NUM / 2 - 1) as f64;
    let v = vars.add_vector(variable().min(min).max(max), BIG_NUM);
    println!("number of variables: {}", v.len());
    let objective: Expression = v.iter().sum();
    let mut prob: CoinCbcProblem = vars.maximise(objective.clone()).using(default_solver);
    let mut n_constraints = 0;
    for vs in v.windows(2) {
        prob = prob.with(constraint!(vs[0] + 1 <= vs[1]));
        n_constraints += 1;
    }
    println!("number of constraints: {}", n_constraints);
    let sol: CoinCbcSolution = prob.solve().unwrap();
    // for (i, var) in v.iter().enumerate() {
    //     assert_float_eq!(sol.value(*var), min + i as f64, abs <= 1e-8);
    // }
    println!("objective: {}", sol.eval(&objective));
}
