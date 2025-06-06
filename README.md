![](./garage.jpeg)
> You have to be kind to the car.<br />
> You feel the poor thing groaning underneath you. <br />
> If you're going to push a piece of machinery to the limit, <br />
> and expect it to hold together, <br />
> you have to have some sense of where that limit is. <br />

`projectcar` is a set of parallel graph algorithms and data structures implemented
in rust. The dynamics of the ownership in rust and the cyclicity of graphs make
it non-trivial to express performant graphs. The primary motivation is to
experiment with various graph representations for the middle-ends of optimizing
compilers which involve lots of mutation via graph rewrites.

<small>This repository was previously the home of typescript implementations which accompanied a [data structure lecture series](https://www.youtube.com/playlist?list=PLn4fTSbSpY5cL4_0MP83wq5khbmG3IKKd). These implementations (located in the [master-old](https://github.com/j4orz/projectcar/tree/master-old) branch) have both correctness and performance issues, but you may find them useful.</small>

**References**
- Sequential: [DCIC](https://dcic-world.org/https://dcic-world.org/), [Erickson](https://jeffe.cs.illinois.edu/teaching/algorithms/), [DPV](http://algorithmics.lsi.upc.edu/docs/Dasgupta-Papadimitriou-Vazirani.pdf), [CP4](https://cpbook.net/)
- Parallel: [Acar and Blelloch](https://www.cs.cmu.edu/~15210/docs/book.pdf), [Acar TAPP](https://www.umut-acar.org/tapp)