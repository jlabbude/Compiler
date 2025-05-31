# Glimpse compiler

Glimpse is a statically, manifested, strongly typed language. With heavy inspiration from C and Rust (the language the compiler is written in).

To use the language, use a file with the `.glim` extension as the first argument.

## Documentation
Find the documentation [here](https://jlabbude.github.io) (PT-BR)

## Examples

- [Example input file](https://github.com/jlabbude/Compiler/blob/main/input.glim)
  - [Output (Lexical analysis)](https://github.com/jlabbude/Compiler/blob/main/output/lexical_output.txt)
  - [Output (Syntax analysis)](https://github.com/jlabbude/Compiler/blob/main/output/syntax_output.csv)

## Example

```C
func int main() {
    print("Hello World!");
    return 0;
}
```

## Roadmap
- [X] Lexical analysis
- [X] Syntax analysis
- [ ] Semantic analysis
