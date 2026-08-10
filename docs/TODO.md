- Support recursive `eval` for nested `Form`s

- Apply given operator for `eval` instead of hardcoded +

- Remove `panic`s in `eval`

- Handle empty list `()` via `List::Empty`

- Support floats via `Atom::Float(f64)`

- Support bools via `Atom::Bool(bool)`

- Support `'` and `quote` to create `List::Data(Data)`

- Support `let` and `lambda`

- Handle escape sequences in strings
