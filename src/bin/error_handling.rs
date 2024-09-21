use good_lp::{constraint, default_solver, solvers::coin_cbc, variables, Solution, SolverModel};

fn main() {
    variables! {
        vars:
               a <= 1;
          2 <= b <= 4;
    } // variables can also be added dynamically
    let objective = 10 * (a - b / 5) - b;
    let prob: coin_cbc::CoinCbcProblem = vars
        .maximise(objective.clone())
        .using(default_solver) // multiple solvers available
        .with(constraint!(a + 2 <= b))
        .with(constraint!(a >= b)) // Raise an error (Infeasible)
        .with(constraint!(1 + a >= 4 - b));
    match prob.solve() {
        Ok(solution) => {
            println!(
                "variables: a={}   b={}",
                solution.value(a),
                solution.value(b)
            );
            println!("objective: {:?}={}", &objective, solution.eval(&objective));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
