use std::i32::{self};

use crate::{
    positions::{
        advance_bit_position::AdvanceBitPosition, array_position::ArrayPosition,
        bit_position::BitPosition, load_starting_position,
    },
    solvers::{
        MAX_SCORE, MIN_SCORE, Solver, alpha_beta_solver::AlphaBetaSolver,
        avoid_losing_moves_solver::AvoidLosingMovesSolver, bitboard_solver::BitBoardSolver,
        center_columns_solver::CenterColumnsSolver,
        iterative_deepening_solver::IterativeDeepeningSolver, move_score_solver::MoveScoreSolver,
        negamax_solver::NegamaxSolver, transposition_table_solver::TranspositionTableSolver,
    },
    transposition_table::TranspositionTable,
};

mod move_sorter;
mod positions;
mod solvers;
mod transposition_table;

const TRANSPOSITION_TABLE_SIZE: usize = 8388593 ;

fn select_board_and_solver(encoded_position: &str) -> Box<dyn Solver> {
    let mut solver_arg: Option<String> = None;

    let mut args = std::env::args().skip(1); // skip binary name

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--solver" => {
                solver_arg = args.next();
            }
            _ => {}
        }
    }

    let table = TranspositionTable::new(TRANSPOSITION_TABLE_SIZE);

    let mut array_position = ArrayPosition::new();
    load_starting_position(encoded_position, &mut array_position);
    let mut bit_position = BitPosition::new();
    load_starting_position(encoded_position, &mut bit_position);
    let mut advance_bit_position = AdvanceBitPosition::new();
    load_starting_position(encoded_position, &mut advance_bit_position);

    let solver: Box<dyn Solver> = match solver_arg.as_deref() {
        Some("negamax") => Box::new(NegamaxSolver::new(array_position)),
        Some("weak-alpha-beta") => Box::new(AlphaBetaSolver::new(array_position, -1, 1)),
        Some("strong-alpha-beta") => {
            Box::new(AlphaBetaSolver::new(array_position, MIN_SCORE, MAX_SCORE))
        }
        Some("weak-center-columns") => Box::new(CenterColumnsSolver::new(array_position, -1, 1)),
        Some("strong-center-columns") => Box::new(CenterColumnsSolver::new(
            array_position,
            MIN_SCORE,
            MAX_SCORE,
        )),
        Some("weak-bitboard") => Box::new(BitBoardSolver::new(bit_position, -1, 1)),
        Some("strong-bitboard") => {
            Box::new(BitBoardSolver::new(bit_position, MIN_SCORE, MAX_SCORE))
        }
        Some("weak-transposition-table") => {
            Box::new(TranspositionTableSolver::new(bit_position, -1, 1, table))
        }
        Some("strong-transposition-table") => Box::new(TranspositionTableSolver::new(
            bit_position,
            MIN_SCORE,
            MAX_SCORE,
            table,
        )),
        Some("weak-iterative-deepening") => {
            Box::new(IterativeDeepeningSolver::new(bit_position, -1, 1, table))
        }
        Some("strong-iterative-deepening") => Box::new(IterativeDeepeningSolver::new(
            bit_position,
            MIN_SCORE,
            MAX_SCORE,
            table,
        )),
        Some("weak-avoid-losing-moves") => Box::new(AvoidLosingMovesSolver::new(
            advance_bit_position,
            -1,
            1,
            table,
        )),
        Some("strong-avoid-losing-moves") => Box::new(AvoidLosingMovesSolver::new(
            advance_bit_position,
            MIN_SCORE,
            MAX_SCORE,
            table,
        )),
        Some("weak-move-score") => {
            Box::new(MoveScoreSolver::new(advance_bit_position, -1, 1, table))
        }
        Some("strong-move-score") => Box::new(MoveScoreSolver::new(
            advance_bit_position,
            MIN_SCORE,
            MAX_SCORE,
            table,
        )),
        Some(other) => panic!("Unknown solver: {}", other),
        None => panic!("Missing --solver argument"),
    };

    solver
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
    time_in_microseconds: usize,
}

fn run(solver: &mut Box<dyn Solver>) -> Metric {
    let start = std::time::Instant::now();
    let score = solver.solve();
    let explored_nodes = solver.explored_nodes();
    let time_in_microseconds = start.elapsed().as_micros() as usize;
    Metric {
        score,
        explored_nodes,
        time_in_microseconds,
    }
}

fn print_metric(metric: &Metric) {
    println!(
        "{} {} {}",
        metric.score, metric.explored_nodes, metric.time_in_microseconds
    )
}

fn main() {
    let encoded_position = read_encoded_position();
    let mut solver = select_board_and_solver(&encoded_position);
    // let table = TranspositionTable::new(TRANSPOSITION_TABLE_SIZE);
    // let mut position = AdvanceBitPosition::new();
    // load_starting_position(&encoded_position, &mut position);
    // let mut solver: Box<dyn Solver> = Box::new(AvoidLosingMovesSolver::new(position, MIN_SCORE, MAX_SCORE, table));
    let metric = run(&mut solver);
    print_metric(&metric);
}
