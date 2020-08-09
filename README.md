
Allocation experiment
===============================

I want to build several trees.

* Parents refer to children, but not the other way around.
* The trees have the 'same' type, but nodes are heterogeneous (enum).
* Different trees have different lifetimes, nodes in the tree have the same lifetime.
* Most nodes refer to a large shared buffer.

For performance:

* I want to intern all strings. Either per tree or globally.
* I want to clean up trees as a whole. Probably no Drop implementations.
* The buffer can be deallocated along with the tree, or after.
* The structure must be mutable, to build the tree (after that it can be immutable).
* Lifetime of the main process is long, so memory for dead trees must be recovered.

Some soft extra requirements:

* Different node types have different sizes (enum variants same size, but they refer to structs of different sizes). Ideally I wouldn't want to waste space.
* I don't want a chance for the nodes to refer to buffer or arena that's gone.
* I'd like to prevent overhead such as Rc.
* I would like nodes that are close together to be mostly adjecent in memory.
* The trees are smaller than 1GB, so using u32 references would save quite some space (not critical).

So some considerations:

* Allocating normally would waste time on allocation and deallocation, and be somewhat fragmented.
* Using bump allocation per tree would have good memory layout and allocation/deallocation performance. Lifetimes would prevent references to freed memory. But I can't get the borrow checker to agree, possibly it's genuinly impossible without aliasing. And the resulting structure cannot be moved because of internal references (even though it's just a _reference to_ such a struct that's being moved).
* Doing the logic in a separate process which has a global bump allocator would incur a expensive transfer to get the data out (possibly data is smaller than whole tree), and processes might be many and live long.
* Using homogeneous buffers (e.g. multiple Vecs) would have good allocation/deallocation performance, build not so great memory locality. References could be u32, though incurs bound check. Problem is: either chance of dangling references, or need an Rc reference to buffer at every index (slow, maybe leaks?).
  => Maybe I could pass the buffers to every operation that uses it, so they couldn't be called after the buffer is gone.


