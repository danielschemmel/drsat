# DRSAT: **D**aniel's **R**usty **SAT** solver

drsat consists of multiple frontends and a library component that actually implements most of the core features.

The currently supported frontends are:
- comp: A barebones implementation of the SATCOMP interface.
- dimacs: A more userfriendly implementation of SAT solver for dimacs files.
- drsat: A meta-frontend that provides the features of dimacs, npn and sudoku, as well as some more candy, as subcommands.
- npn: A SAT solver for a special, simplified format [draft phase].
- sudoku: Solves sudoku puzzles by way of generating and solving an equivalent SAT query. Optionally does additional simplifications based on sudoku rules or provides the generated query.

Features of the core SAT solver:
- Context Driven Clause Learning (CDCL) solver for Conjunctive Normal Form (CNF) SAT queries
- Two watched literals clause watchlists
- Geometric garbage collection based on clause glues
- Geometric restarts (in combination with garbage collection)
- Learnt clause minimization
- Phase saving
- Conflict History-Based (CHB) branching heuristic, a variant on the Exponential Recency Weighted Average (ERWA) branching heuristic
- Initialization of branching scores based on static heuristic
- Basic preprocessing

(c) by Daniel Schemmel, unlicensed for any use