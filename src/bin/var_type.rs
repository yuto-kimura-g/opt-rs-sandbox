// https://docs.rs/good_lp/latest/good_lp/macro.variables.html

use good_lp::{
    default_solver, solvers::coin_cbc::CoinCbcProblem, variables, Expression, Solution, SolverModel,
};

fn main() {
    variables! {
        vars:
            1 <= x <= 10;
            0 <= v[3] <= 1; // x will be a vector of 3 variables
            1 <= z (integer) <= 10;
            b (binary);
    };
    let objective: Expression = v.iter().sum();
    let prob: CoinCbcProblem = vars.maximise(objective.clone()).using(default_solver);
    match prob.solve() {
        Ok(sol) => {
            println!("objective: {:?}={}", &objective, sol.eval(&objective));
            println!("x={}", sol.value(x));
            println!(
                "v={:?}",
                v.iter().map(|&vi| sol.value(vi)).collect::<Vec<_>>()
            );
            println!("z={}", sol.value(z));
            println!("b={}", sol.value(b));
        }
        Err(e) => println!("Error: {}", e),
    }
}
