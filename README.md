![](./garage.jpeg)
> You have to be kind to the car.<br />
> You feel the poor thing groaning underneath you. <br />
> If you're going to push a piece of machinery to the limit, <br />
> and expect it to hold together, <br />
> you have to have some sense of where that limit is. <br />

`projectcar` reimplements [`std::collections`]() plus concurrent plus graphs plus
custom allocators using the nightly-only experimental [`allocator_api`](https://doc.rust-lang.org/std/alloc/trait.Allocator.html). This repository was previously the home of typescript implementations
which accompanied a
[data structure lecture series](https://www.youtube.com/playlist?list=PLn4fTSbSpY5cL4_0MP83wq5khbmG3IKKd).
These implementations (located in the [master-old](https://github.com/j4orz/projectcar/tree/master-old) branch)
have both correctness and performance issues, but you may find them useful.

## Allocators
- `Slab`
- `Stack`
- `Buddy`

## Algorithms + Data Structures = Programs
**Sequences: `Vec`, `VecDeque`, `LinkedList`**
*Analytical asymptotics*
*Emprical profiles*

**Graphs: `AdjacencyList`, `AdjacencyMatrix`, `EdgeList`**
*Analytical asymptotics*
*Emprical profiles*

**Other: `HashMap`, `HashSet`, `BinaryHeap`**
*Analytical asymptotics*
*Emprical profiles*

## References
- Rust: [trpl](https://rust-book.cs.brown.edu/), [std::collections](https://doc.rust-lang.org/std/collections/), [reference](https://doc.rust-lang.org/reference/), [nomicon](https://doc.rust-lang.org/nomicon/), [lrwetmll](https://rust-unofficial.github.io/too-many-lists/index.html)
- Sequential: [DCIC](https://dcic-world.org/https://dcic-world.org/), [Erickson](https://jeffe.cs.illinois.edu/teaching/algorithms/), [DPV](http://algorithmics.lsi.upc.edu/docs/Dasgupta-Papadimitriou-Vazirani.pdf), [KT](https://www.cs.princeton.edu/~wayne/kleinberg-tardos/), [CP4](https://cpbook.net/)
- Parallel: [Acar and Blelloch](https://www.cs.cmu.edu/~15210/docs/book.pdf), [Acar TAPP](https://www.umut-acar.org/tapp), [AMP](https://www.amazon.ca/Art-Multiprocessor-Programming-Maurice-Herlihy/dp/0123705916)