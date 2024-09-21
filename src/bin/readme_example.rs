// https://github.com/rust-or/good_lp/blob/main/tests/readme_example.rs

use std::error::Error;

use good_lp::{
    constraint, default_solver, solvers::coin_cbc, variables, Expression, Solution, SolverModel,
};

fn main() -> Result<(), Box<dyn Error>> {
    variables! {
        vars:
               a <= 1;
          2 <= b <= 4;
    } // variables can also be added dynamically
    let objective: Expression = 10 * (a - b / 5) - b;
    let solution: coin_cbc::CoinCbcSolution = vars
        .maximise(objective.clone())
        .using(default_solver) // multiple solvers available
        .with(constraint!(a + 2 <= b))
        .with(constraint!(1 + a >= 4 - b))
        .solve()?;
    println!(
        "variables: a={}   b={}",
        solution.value(a),
        solution.value(b)
    );
    println!("objective: {:?}={}", &objective, solution.eval(&objective));
    Ok(())
}
