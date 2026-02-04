use crate::{positions::{Position, array_position::ArrayPosition}, solvers::{Solver, negamax_solver::NegamaxSolver}};

mod positions;
mod solvers;

fn select_board_and_solver() -> (impl Position, impl Solver) {
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

    (
    match board_arg.as_deref() {
        Some("array") => ArrayPosition::new(),
        Some(other) => panic!("Unknown board: {}", other),
        None => panic!("Missing --board argument"),
    },
    match solver_arg.as_deref() {
        Some("negamax") => NegamaxSolver::new(),
        Some(other) => panic!("Unknown solver: {}", other),
        None => panic!("Missing --solver argument"),
    }
)
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

fn run<P: Position, S:Solver>(position: &mut P, solver: &mut S) -> Metric{
    let start = std::time::Instant::now();
    let score = solver.solve(position);
    let explored_nodes = solver.explored_nodes();
    let time_in_microseconds = start.elapsed().as_millis() as usize;
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
