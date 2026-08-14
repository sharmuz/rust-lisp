- ~~Support `eval` of nested `Form`s via recursion~~

- ~~Apply given operator for `eval` instead of hardcoded +~~

- ~~Remove remaining `panic`s in `eval`~~

- Handle `Expr` that are a single `Atom::String(s)`
    - Via a custom `LispValue` type?

- Handle empty list `()` via `List::Empty`

- Support floats via `Atom::Float(f64)`

- Support bools via `Atom::Bool(bool)`

- Support `'` and `quote` to create `List::Data(Data)`

- Support `lambda` for definitions

- Support `define` for persistent variables

- Support `let` for scoped variables

- Handle escape sequences in strings
