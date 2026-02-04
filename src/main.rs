use std::i32;

use crate::{positions::{Position, array_position::ArrayPosition}, solvers::{MAX_SCORE, MIN_SCORE, Solver, alpha_beta_solver::AlphaBetaSolver, negamax_solver::NegamaxSolver}};

mod positions;
mod solvers;

fn select_board_and_solver() -> (Box<dyn Position>, Box<dyn Solver>) {
    let mut board_arg: Option<String> = None;
    let mut solver_arg: Option<String> = None;

    let mut args = std::env::args().skip(1); // skip binary name

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--board" => {
                board_arg = args.next();
            }
            "--solver" => {
                solver_arg = args.next();
            }
            _ => {}
        }
    }

    let position: Box<dyn Position> =  match board_arg.as_deref() {
        Some("array") =>Box::new(ArrayPosition::new()),
        Some(other) => panic!("Unknown board: {}", other),
        None => panic!("Missing --board argument"),
    };

    let solver: Box<dyn Solver>= match solver_arg.as_deref() {
        Some("negamax") => Box::new(NegamaxSolver::new()),
        Some("weak-alpha-beta") => Box::new(AlphaBetaSolver::new(-1, 1)),
        Some("strong-alpha-beta") => Box::new(AlphaBetaSolver::new(MIN_SCORE, MAX_SCORE)),
        Some(other) => panic!("Unknown solver: {}", other),
        None => panic!("Missing --solver argument"),
    };

    (position, solver)
}

fn read_encoded_position() -> String {
    let mut encoded_position = String::new();
    std::io::stdin()
        .read_line(&mut encoded_position)
        .expect("couldn't read the encoded position");
    encoded_position.trim().to_string()
}

struct Metric {
    score: i32,
    explored_nodes: usize,
    time_in_microseconds: usize
}

fn run(position: &mut Box<dyn Position>, solver: &mut Box<dyn Solver>) -> Metric{
    let start = std::time::Instant::now();
    let score = solver.solve(position);
    let explored_nodes = solver.explored_nodes();
    let time_in_microseconds = start.elapsed().as_micros() as usize;
    Metric { score, explored_nodes, time_in_microseconds }
}

fn print_metric(metric: &Metric) {
    println!("{} {} {}", metric.score, metric.explored_nodes, metric.time_in_microseconds)
}

fn main() {
    let (mut position, mut solver) = select_board_and_solver();
    // let (mut position, mut solver) = (ArrayPosition::new(), NegamaxSolver::new());
    let encoded_position = read_encoded_position();
    positions::load_starting_position(&encoded_position, &mut position);
    let metric = run(&mut position, & mut solver);
    print_metric(&metric);
}
