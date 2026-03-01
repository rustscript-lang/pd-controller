# Compiler Notes

This document tracks current compiler behavior and known subset limitations.

## Not Supported

### Core compiler/IR limitations

- No first-class function values in IR/runtime call path.
  - Calls are lowered as static call indices (`Expr::Call` + `call` opcode), not callable values.
  - Closures in this subset are not general values: they must be bound/called through closure binding rules.
  - Higher-order patterns like "store function in data", "return function value", and "call arbitrary expression" are not generally supported.
- Recursive RustScript function declarations are not supported in current inlining-based lowering (recursive calls error during compile).
- Nested function declarations are not supported.
- RustScript function declarations cannot capture outer locals.
- `match` patterns are currently limited to int/string literals and `_`.
- `break`/`continue` are only valid inside loops.
- Host import namespace support in compiler parser is limited to `vm` (plus `io::` builtin namespace calls where applicable).

### Module/source loading limitations

- `crate::...` module paths are not supported in RustScript source loading; use relative module paths.

### JavaScript frontend subset limitations

- Arrow closures with block bodies are not supported (expression-body arrows only).
- Empty arrow parameter lists are not supported in this subset.

### Lua frontend subset limitations

- Numeric `for` loops with negative step are not supported.
- Lua pattern API string methods (`:match`, `:gsub`, etc.) are not supported in this subset.
- Function literals are restricted by the direct lowering subset and require a non-empty parameter list and non-empty return expression.

### Scheme frontend subset limitations

- No runtime symbol/procedure type support in VM typing model:
  - `procedure?` always lowers to `false`.
  - `symbol?` always lowers to `false`.
- `string->number` currently lowers to placeholder behavior (`0`) due to missing parse builtin.
- `apply` is limited to `(apply func arglist)` in this subset and does not provide proper spread/varargs semantics.
