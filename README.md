# opt-rs-playground

Mathematical Optimization with Rust: playground

## install
```bash
sudo apt update
sudo apt install coinor-cbc coinor-libcbc-dev
cargo add good_lp
```

```bash
$ cbc --version

Welcome to the CBC MILP Solver
Version: 2.10.7
Build Date: Feb 14 2022
```

## run
```bash
$ cargo run --bin readme_example

Presolve 2 (0) rows, 2 (0) columns and 4 (0) elements
0  Obj 4 Primal inf 0.9999999 (1)
1  Obj 1
Optimal - objective value 1
Optimal objective 1 - 1 iterations time 0.002
variables: a=1   b=3
objective: -3 v1 + 10 v0=1
```

```bash
$ time cargo run --bin scaling

number of variables: 1000000
number of constraints: 999999
Presolve 0 (-999999) rows, 0 (-1000000) columns and 0 (-1999998) elements
Optimal - objective value -500000
After Postsolve, objective -500000, infeasibilities - dual 0 (0), primal 0 (0)
Optimal objective -500000 - 0 iterations time 2.962, Presolve 2.96
objective: -500000

real    0m26.230s
user    0m19.789s
sys     0m5.757s
```

```bash
$ cargo run --bin error_handling

Presolve determined that the problem was infeasible with tolerance of 1e-08
Analysis indicates model infeasible or unbounded
0  Obj 4 Primal inf 1.9999998 (2)
Primal infeasible - objective value 4
PrimalInfeasible objective 4 - 0 iterations time 0.012
Error: Infeasible: The problem contains contradictory constraints. No solution exists.
```

```bash
$ cargo run --bin var_type

Welcome to the CBC MILP Solver
Version: 2.10.7
Build Date: Feb 14 2022

command line - Cbc_C_Interface -solve -quit (default strategy 1)
Continuous objective value is 3 - 0.00 seconds
Cgl0004I processed model has 0 rows, 0 columns (0 integer (0 of which binary)) and 0 elements
Cbc3007W No integer variables - nothing to do
Cuts at root node changed objective from -3 to -1.79769e+308
Probing was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
Gomory was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
Knapsack was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
Clique was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
MixedIntegerRounding2 was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
FlowCover was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
TwoMirCuts was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)
ZeroHalf was tried 0 times and created 0 cuts of which 0 were active after adding rounds of cuts (0.000 seconds)

Result - Optimal solution found

Objective value:                3.00000000
Enumerated nodes:               0
Total iterations:               0
Time (CPU seconds):             0.01
Time (Wallclock seconds):       0.01

Total time (CPU seconds):       0.01   (Wallclock seconds):       0.01

objective: v1 + v3 + v2=3
x=1
v=[1.0, 1.0, 1.0]
z=1
b=0
```

## references
- <https://github.com/rust-or/good_lp>
- <https://docs.rs/good_lp/latest/good_lp/index.html>
- <https://crates.io/crates/good_lp>
