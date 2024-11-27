# Glimpse compiler

Glimpse is a statically, manifested, strongly typed language. With heavy inspiration from C and Rust (the language the compiler is written in).

To use the language, use a file with the `.glim` extension as an argument when executing the [executable](https://github.com/jlabbude/Compiler/releases/tag/exe).

## Documentation
Find the documentation [here](https://jlabbude.github.io) (PT-BR)

## Examples

- [Example input file](https://github.com/jlabbude/Compiler/blob/main/input.nh)
  - [Output (Lexical analysis)](https://github.com/jlabbude/Compiler/blob/main/output/output.txt)
  - [Organized tokens (CSV)](https://github.com/jlabbude/Compiler/blob/main/output/output.csv)

## Example

```C
func int main() {
    print("Hello World!");
    return 0;
}
```

## Roadmap
- [X] Lexical analysis
- [ ] Syntax analysis (5%)
- [ ] Semantic analysis