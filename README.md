# Rust Vector Modification Bug

This repository demonstrates a common error in Rust programming involving modifying a vector while holding a reference to one of its elements. The code in `bug.rs` attempts to print the value of an element while adding new elements to the vector, which leads to a runtime panic.

The solution in `bugSolution.rs` provides a way to fix this by avoiding the issue of holding the reference while modifying the vector.