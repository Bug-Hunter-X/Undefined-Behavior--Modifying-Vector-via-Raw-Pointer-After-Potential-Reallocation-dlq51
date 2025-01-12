# Rust Undefined Behavior Example

This repository demonstrates a common source of undefined behavior in Rust: modifying a vector through a raw pointer after the vector may have reallocated.  This can lead to crashes or unpredictable behavior.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file provides a safer, correct alternative.

**Key Concepts**

* **Ownership and Borrowing:** Rust's ownership system prevents many common memory-related errors. Raw pointers bypass this system, requiring careful handling.
* **Vector Reallocation:** Vectors in Rust can reallocate their memory when they grow beyond their capacity.  This invalidates any existing pointers to the vector's data.

**How to Reproduce**

1. Clone this repository.
2. Navigate to the project directory.
3. Run `rustc bug.rs` and then `./bug` to see the undefined behavior (may crash or produce unexpected output).
4. Run `rustc bugSolution.rs` and then `./bugSolution` to see the corrected version. 