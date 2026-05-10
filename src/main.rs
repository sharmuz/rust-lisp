fn main() {
    let input = "(first (list 1 (+ 2 3) 9))";
    println!("Input is: {input}");
    let tokens = tokenize(input);
    println!("Tokens are: {tokens:?}");
    let expr = Expr::from(tokens);
    println!("Expression is: {expr:?}");
}

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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Atom(Atom),
    List(List),
}

#[allow(clippy::fallible_impl_from)]
impl From<Vec<Token>> for Expr {
    fn from(tokens: Vec<Token>) -> Self {
        match &tokens[0] {
            Token::Atom(a) => {
                if tokens.len() > 1 {
                    panic!("Invalid expr: atom must occur alone")
                } else {
                    Self::Atom(a.clone())
                }
            }
            Token::OpenBracket => {
                assert!(
                    tokens.last() == Some(&Token::CloseBracket),
                    "Invalid expr: missing final closing bracket"
                );
                let op = if let Token::Atom(a) = &tokens[1] {
                    a.clone()
                } else {
                    panic!("Invalid expr: form is missing operator")
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
                                let next_arg = Self::from(partial_arg.clone());
                                args.push(next_arg);
                                partial_arg.clear();
                            }
                        }
                    }
                }
                Self::List(List::Form(Form { operator: op, args }))
            }
            Token::CloseBracket => {
                panic!("Invalid expr: cannot start with closing bracket")
            }
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
        let expr = Expr::from(tokens);

        assert_eq!(expr, expected);
    }

    #[test]
    fn atom_from_double_quoted_string() {
        let input = "\"banana\"".to_string();
        let expected = Atom::String("banana".to_string());
        let atom = Atom::from(input);

        assert_eq!(atom, expected);
    }
}
