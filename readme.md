# DRSAT: **D**aniel's **R**usty **SAT** solver

drsat consists of multiple frontends and a library component that actually implements most of the core features.

The currently supported frontends are:
- comp: A barebones implementation of the SATCOMP interface.
- dimacs: A more userfriendly implementation of SAT solver for dimacs files.
- drsat: A meta-frontend that provides the features of dimacs, npn and sudoku, as well as some more candy, as subcommands.
- npn: A SAT solver for a special, simplified format.
- sudoku: Solves sudoku puzzles by way of generating and solving an equivalent SAT query. Optionally does additional simplifications based on sudoku rules or provides the generated query.