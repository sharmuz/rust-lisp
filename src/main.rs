use std::error::Error;

fn main() -> Result<(), LispError> {
    let input = "(- (+ 9 (/ 6 1)) (* 2 (/ 5 2)))";
    println!("Input is: {input}");
    let tokens = tokenize(input);
    println!("Tokens are: {tokens:?}");
    let expr = Expr::try_from(tokens)?;
    println!("Expression is: {expr:?}");
    let res = expr.eval()?;
    println!("Result is: {res}");
    Ok(())
}

/// Generates a sequence of tokens representing the input expression.
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut t: Vec<char> = Vec::new();
    let brackets = &['(', ')'];
    let mut is_string = false;

    for ch in input.chars() {
        if ch == '"' {
            is_string = !is_string;
        }
        let is_delimeter = brackets.contains(&ch) || ch.is_whitespace();
        if is_delimeter && !is_string {
            if !t.is_empty() {
                tokens.push(Token::from(t.clone()));
                t.clear();
            }
            if brackets.contains(&ch) {
                tokens.push(Token::from(vec![ch]));
            }
        } else {
            t.push(ch);
        }
    }
    if !t.is_empty() {
        tokens.push(Token::from(t.clone()));
    }

    tokens
}

/// A LISP S-expression.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Atom(Atom),
    List(List),
}

impl Expr {
    fn eval(&self) -> Result<isize, LispError> {
        match self {
            Self::Atom(Atom::Number(n)) => Ok(*n),
            Self::Atom(Atom::String(_)) => Err(LispError::Unsupported("String eval".into())),
            Self::Atom(Atom::Symbol(s)) => Err(LispError::Eval(format!("Symbol `{s}`"))),
            Self::List(List::Form(f)) => match &f.operator {
                Atom::Symbol(s) => {
                    let mut args = f.args.iter().map(Self::eval);
                    let first = args
                        .next()
                        .unwrap_or_else(|| Err(LispError::Eval("Form is missing args".into())));
                    args.try_fold(first?, |acc, x| match s.as_str() {
                        "+" => Ok(acc + x?),
                        "-" => Ok(acc - x?),
                        "*" => Ok(acc * x?),
                        "/" => Ok(acc / x?),
                        _ => Err(LispError::Eval(format!("Unknown operator `{s}`"))),
                    })
                }
                _ => Err(LispError::Eval(format!(
                    "Invalid operator `{:?}`",
                    f.operator,
                ))),
            },
            Self::List(List::Data(_)) => Err(LispError::Unsupported("List eval".into())),
        }
    }
}

/// Creates an Expr from a sequence of Tokens.
#[allow(clippy::fallible_impl_from)]
impl TryFrom<Vec<Token>> for Expr {
    type Error = LispError;

    fn try_from(tokens: Vec<Token>) -> Result<Self, Self::Error> {
        match &tokens[0] {
            Token::Atom(a) => {
                if tokens.len() > 1 {
                    Err(LispError::Parse("Atom must occur alone".into()))
                } else {
                    Ok(Self::Atom(a.clone()))
                }
            }
            Token::OpenBracket => {
                assert!(
                    tokens.last() == Some(&Token::CloseBracket),
                    "Invalid expr: missing final closing bracket"
                );
                let op = if let Token::Atom(a) = &tokens[1] {
                    Ok(a.clone())
                } else {
                    Err(LispError::Parse("Form is missing operator".into()))
                };

                let mut args: Vec<Self> = Vec::new();
                let mut partial_arg: Vec<Token> = Vec::new();
                let mut num_open_brackets = 0;

                for t in tokens.iter().skip(2).take(tokens.len() - 3) {
                    match t {
                        Token::Atom(a) => {
                            if num_open_brackets == 0 {
                                args.push(Self::Atom(a.clone()));
                            } else {
                                partial_arg.push(t.clone());
                            }
                        }
                        Token::OpenBracket => {
                            num_open_brackets += 1;
                            partial_arg.push(t.clone());
                        }
                        Token::CloseBracket => {
                            num_open_brackets -= 1;
                            partial_arg.push(t.clone());

                            if num_open_brackets == 0 {
                                let next_arg = Self::try_from(partial_arg.clone())?;
                                args.push(next_arg);
                                partial_arg.clear();
                            }
                        }
                    }
                }
                Ok(Self::List(List::Form(Form {
                    operator: op?,
                    args,
                })))
            }
            Token::CloseBracket => Err(LispError::Parse(
                ("Cannot start with closing bracket").into(),
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum List {
    Form(Form),
    Data(Data),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Form {
    operator: Atom,
    args: Vec<Expr>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Data(Vec<Expr>);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Atom(Atom),
}

impl From<Vec<char>> for Token {
    fn from(chars: Vec<char>) -> Self {
        if chars.len() == 1 {
            if chars[0] == '(' {
                return Self::OpenBracket;
            }
            if chars[0] == ')' {
                return Self::CloseBracket;
            }
        }

        let as_string = chars.into_iter().collect::<String>();
        Self::Atom(Atom::from(as_string))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Atom {
    Number(isize),
    String(String),
    Symbol(String),
}

impl From<String> for Atom {
    fn from(input: String) -> Self {
        match (input.chars().nth(0), input.chars().last()) {
            (Some(x), Some(y)) if x == '"' && y == '"' => {
                Self::String(input.chars().skip(1).take(input.len() - 2).collect())
            }
            _ => input
                .parse::<isize>()
                .map_or_else(|_e| Self::Symbol(input.clone()), Self::Number),
        }
    }
}

#[derive(Debug)]
enum LispError {
    Eval(String),
    Parse(String),
    Unsupported(String),
}

impl std::fmt::Display for LispError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eval(s) => write!(f, "Failed to evaluate: {s}"),
            Self::Parse(s) => write!(f, "Failed to parse: {s}"),
            Self::Unsupported(s) => write!(f, "Unsupported: {s}"),
        }
    }
}

impl Error for LispError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_single_atom() {
        let input = "28";
        let expected = vec![Token::Atom(Atom::Number(28))];
        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn tokenize_basic_form() {
        let input = "(* 3 (+ 1 2))";
        let expected = vec![
            Token::OpenBracket,
            Token::Atom(Atom::Symbol("*".to_string())),
            Token::Atom(Atom::Number(3)),
            Token::OpenBracket,
            Token::Atom(Atom::Symbol("+".to_string())),
            Token::Atom(Atom::Number(1)),
            Token::Atom(Atom::Number(2)),
            Token::CloseBracket,
            Token::CloseBracket,
        ];
        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn tokenize_string_containing_delims() {
        let input = "(print \"hello world :)\")";
        let expected = vec![
            Token::OpenBracket,
            Token::Atom(Atom::Symbol("print".to_string())),
            Token::Atom(Atom::String("hello world :)".to_string())),
            Token::CloseBracket,
        ];
        let tokens = tokenize(input);

        assert_eq!(tokens, expected);
    }

    #[test]
    fn expr_from_nested_forms() {
        let tokens = vec![
            Token::OpenBracket,
            Token::Atom(Atom::Symbol("first".to_string())),
            Token::OpenBracket,
            Token::Atom(Atom::Symbol("list".to_string())),
            Token::Atom(Atom::Number(1)),
            Token::OpenBracket,
            Token::Atom(Atom::Symbol("+".to_string())),
            Token::Atom(Atom::Number(2)),
            Token::Atom(Atom::Number(3)),
            Token::CloseBracket,
            Token::Atom(Atom::Number(9)),
            Token::CloseBracket,
            Token::CloseBracket,
        ];
        let expected = Expr::List(List::Form(Form {
            operator: Atom::Symbol("first".to_string()),
            args: vec![Expr::List(List::Form(Form {
                operator: Atom::Symbol("list".to_string()),
                args: vec![
                    Expr::Atom(Atom::Number(1)),
                    Expr::List(List::Form(Form {
                        operator: Atom::Symbol("+".to_string()),
                        args: vec![Expr::Atom(Atom::Number(2)), Expr::Atom(Atom::Number(3))],
                    })),
                    Expr::Atom(Atom::Number(9)),
                ],
            }))],
        }));
        let expr = Expr::try_from(tokens).expect("Tokens should be parseable");

        assert_eq!(expr, expected);
    }

    #[test]
    fn atom_from_double_quoted_string() {
        let input = "\"banana\"".to_string();
        let expected = Atom::String("banana".to_string());
        let atom = Atom::from(input);

        assert_eq!(atom, expected);
    }

    #[test]
    fn eval_nested_addition() {
        let input = "(+ (+ 3 6) 7)";
        let expected = 16;
        let tokens = tokenize(input);
        let eval = Expr::try_from(tokens)
            .expect("Tokens should be parseable")
            .eval();

        assert_eq!(eval.expect("Expr should eval to isize"), expected);
    }

    #[test]
    fn eval_nested_multi_ops() {
        let input = "(- (* 12 3) (+ 3 (/ 17 4)) 2)";
        let expected = 27;
        let tokens = tokenize(input);
        let eval = Expr::try_from(tokens)
            .expect("Tokens should be parseable")
            .eval();

        assert_eq!(eval.expect("Expr should eval to isize"), expected);
    }
}
