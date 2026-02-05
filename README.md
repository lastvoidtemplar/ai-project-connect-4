# Connect 4 Agent

## Datasets

The project uses 6 datasets to benchmark the agent

| Dataset ID(1000 cases) | Dataset name |Starting position length(number of already played moves) | Solution Depth(number of remaining moves) |
| :--- | :--- | :--- | :--- |
| [Test_L1_R1](http://blog.gamesolver.org/data/Test_L3_R1) | End-Easy | 28 < moves | remaining < 14 |
| [Test_L2_R1](http://blog.gamesolver.org/data/Test_L2_R1) | Middle-Easy | 	14 < moves <= 28 | remaining < 14 |
| [Test_L2_R2](http://blog.gamesolver.org/data/Test_L2_R2) | Middle-Medium | 14 < moves <= 28 | 14 <= remaining < 28 |
| [Test_L3_R1](http://blog.gamesolver.org/data/Test_L3_R1) | End-Easy | moves <= 14 | remaining < 14 |
| [Test_L3_R2](http://blog.gamesolver.org/data/Test_L3_R2) | End-Medium | moves <= 14 | 14 <= remaining < 28 |
| [Test_L3_R3](http://blog.gamesolver.org/data/Test_L3_R3) | End-Hard | moves <= 14 | 28 <= remaining |

## Run benchmark

strong solver - negamax with array board with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_negamax.csv --timeout 10 -- ./target/release/project --solver negamax
```

weak solver - alpha beta pruning negamax with array board with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_weak_alpha_beta.csv --timeout 10 -- ./target/release/project --solver weak-alpha-beta
```

strong solver - alpha beta pruning negamax with array board with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_strong_alpha_beta.csv --timeout 10 -- ./target/release/project --solver strong-alpha-beta
```

weak solver - alpha beta pruning negamax with array board and center columns heuristic with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_weak_center_columns.csv --timeout 10 -- ./target/release/project --solver weak-center-columns
```

strong solver - alpha beta pruning negamax with array board and center columns heuristicwith 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_strong_center_columns.csv --timeout 10 -- ./target/release/project --solver strong-center-columns
```

weak solver - alpha beta pruning negamax with bitboard and center columns heuristic with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_weak_bitboard.csv --timeout 10 -- ./target/release/project --solver weak-bitboard
```

strong solver - alpha beta pruning negamax with bitboard and center columns heuristicwith 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_strong_bitboard.csv --timeout 10 -- ./target/release/project --solver strong-bitboard
```

weak solver - alpha beta pruning negamax with bitboard and center columns heuristic and transposition table with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_weak_transposition_table.csv --timeout 10 -- ./target/release/project --solver weak-transposition-table
```

strong solver - alpha beta pruning negamax with bitboard and center columns heuristic and transposition table with 10 seconds for each case
```bash
python3 scripts/benchmark/benchmark.py --dir ./data --out result_strong_transposition_table.csv --timeout 10 -- ./target/release/project --solver strong-transposition-table
```