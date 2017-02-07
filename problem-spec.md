
# Shape and Memory: 
* Integer list operations translate into array transformations (concatenate => outer product,
  permutation => transpose, eliminate duplicates => inner product)
* Indexing contiguous memory can be done with a linear form in homogenous coordinates, allowing
  easy slicing.

# Lazyness and Strictness:
* Slicing is an example of lazyness, saves memory
* Sparse arrays are lazy
* Memoization + sparse for outer products of large arrays
* Strictness has better cache performance?

# Implementation in Rust:
* Basic array types, generic traits, operations
* Possibly procedural macro DSL for fusion of operations?
