# DRSAT: **D**aniel's **R**usty **SAT** solver

drsat consists of multiple frontends and a library component that actually implements most of the core features.

The following frontends are provided as part of drsat:
- `comp`: A barebones implementation of the SATCOMP interface.
- `dimacs`: Another implementation of a SAT solver for dimacs files, which is more user friendly than `comp`.
- `drsat`: A meta-frontend that provides the features of dimacs, npn and sudoku, as well as some more candy, as subcommands.
- `npn`: A SAT solver for a special, simplified format [draft phase].
- `sudoku`: Solves sudoku puzzles by way of generating and solving an equivalent SAT query. Optionally does simple additional simplifications based on sudoku rules. It can provide the generated query.

Features of the core SAT solver:
- Context Driven Clause Learning (CDCL) solver for Conjunctive Normal Form (CNF) SAT queries
- Two watched literals clause watchlists
- Geometric learnt clause deletion based on clause glues
- Geometric restarts (in conjunction with learnt clause deletion)
- Learnt clause minimization
- Phase saving
- Conflict History-Based (CHB) branching heuristic, a Exponential Recency Weighted Average (ERWA) branching heuristic
- Initialization of CHB scores based on an additional static heuristic
- Basic preprocessing

While drsat is written in a way that ensures that available memory and time are the only limits to which queries can be solved, it is possible to enable additional optimizations by enabling the `aggressive` feature, such as in `cargo build --release --features=aggressive`. This will for example reduce the number of supported variables to about 2 billion. While the optimizations enabled this way should not cause any troubly in the general case, they are not enabled by default to emphasize that they may technically cause problems.

Unlike most state of the art SAT solvers such as minisat or lingeling (and their various derivatives), drsat is written in pure, safe Rust excepting only external libraries used e.g. when opening compressed files. When porting the original C++ version to Rust in January of 2017, unscientific, exploratory measurements gave a roughly 15% decrease in performance (note that both compilers as well as drsat itself have changed - and hopefully improved - since then). Most of that cost probably comes from runtime bounds checking.

While drsat itself is an original program not derived from any other SAT solver, its algorithms are of course mostly not original inventions.

(c) 2017 Daniel Schemmel, all rights reserved