Integration with miniZinc directory: /src/zinc

MiniZinc can export constraint satisfaction problem to FlatZinc format (*.fzn):
	- https://docs.minizinc.dev/en/stable/flattening.html
	- fzn specification: https://docs.minizinc.dev/en/latest/fzn-spec.html  BNF grammar syntax at document end.


FlatZinc examples: https://github.com/google/or-tools/tree/stable/examples/flatzinc
Examples explanation: https://www.hakank.org/minizinc/
Local examples are in /src/zinc/flatzinc

1. Import .fzn model.
- Not sure what is the latest spec version? The MiniZinc latest release: 2.9.4

2. Create AST.

3. Map AST to programmable Selen API.

4. Solve problem.

5. Output result/error in FlatZinc format

