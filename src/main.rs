fn main() {
    let lisp_expr = "(first (list 1 (+ 2 3) 9))";
    let tokens = tokenize(lisp_expr);
    println!("{tokens:?}");
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut t: Vec<char> = Vec::new();
    let brackets = &['(', ')'];

    for ch in input.chars() {
        if brackets.contains(&ch) || ch.is_whitespace() {
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
    fn atom_from_double_quoted_string() {
        let input = "\"banana\"".to_string();
        let expected = Atom::String("banana".to_string());
        let atom = Atom::from(input);

        assert_eq!(atom, expected);
    }
}
