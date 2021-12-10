use std::io::BufRead;

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenAngle,
    CloseAngle,
}

#[derive(Debug, PartialEq)]
enum LineStatus {
    Valid,
    Corrupt { expected: Token, illegal: Token },
    Incomplete { expected: Vec<Token> },
}

impl LineStatus {
    fn score(&self) -> usize {
        match self {
            LineStatus::Incomplete { expected } => {
                let mut score = 0;

                for expected_token in expected {
                    score *= 5;

                    match expected_token {
                        Token::CloseParen => score += 1,
                        Token::CloseSquare => score += 2,
                        Token::CloseCurly => score += 3,
                        Token::CloseAngle => score += 4,
                        token => panic!("Encountered invalid token: {:?}", token),
                    }
                }

                score
            }
            _ => 0,
        }
    }
}

fn lex(s: &str) -> Vec<Token> {
    let s = s.trim();
    s.chars()
        .map(|c| match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '[' => Token::OpenSquare,
            ']' => Token::CloseSquare,
            '{' => Token::OpenCurly,
            '}' => Token::CloseCurly,
            '<' => Token::OpenAngle,
            '>' => Token::CloseAngle,
            _ => panic!("Invalid character"),
        })
        .collect()
}

fn parse(tokens: &[Token]) -> LineStatus {
    let mut counter = 0;
    let mut expected = vec![];

    for token in tokens {
        match token {
            Token::OpenParen => {
                expected.push(Token::CloseParen);
                counter += 1;
            }
            Token::CloseParen => {
                if let Some(t) = expected.pop() {
                    if !matches!(t, Token::CloseParen) {
                        return LineStatus::Corrupt {
                            expected: t,
                            illegal: Token::CloseParen,
                        };
                    }
                }
                counter -= 1;
            }
            Token::OpenSquare => {
                expected.push(Token::CloseSquare);
                counter += 1;
            }
            Token::CloseSquare => {
                if let Some(t) = expected.pop() {
                    if !matches!(t, Token::CloseSquare) {
                        return LineStatus::Corrupt {
                            expected: t,
                            illegal: Token::CloseSquare,
                        };
                    }
                }
                counter -= 1;
            }
            Token::OpenCurly => {
                expected.push(Token::CloseCurly);
                counter += 1;
            }
            Token::CloseCurly => {
                if let Some(t) = expected.pop() {
                    if !matches!(t, Token::CloseCurly) {
                        return LineStatus::Corrupt {
                            expected: t,
                            illegal: Token::CloseCurly,
                        };
                    }
                }
                counter -= 1;
            }
            Token::OpenAngle => {
                expected.push(Token::CloseAngle);
                counter += 1;
            }
            Token::CloseAngle => {
                if let Some(t) = expected.pop() {
                    if !matches!(t, Token::CloseAngle) {
                        return LineStatus::Corrupt {
                            expected: t,
                            illegal: Token::CloseAngle,
                        };
                    }
                }
                counter -= 1;
            }
        }
    }

    if counter != 0 {
        expected.reverse();

        return LineStatus::Incomplete { expected };
    }

    LineStatus::Valid
}

fn main() {
    let input = std::fs::File::open("inputs/10.txt").unwrap();
    let lines = std::io::BufReader::new(input).lines();

    let file_lines = lines.map(|line| line.unwrap());

    let mut scores = vec![];

    for line in file_lines {
        let lexed = lex(&line);
        let line_status = parse(&lexed);
        if let LineStatus::Incomplete { .. } = line_status {
            let score = line_status.score();
            scores.push(score);
        };
    }

    scores.sort_unstable();
    let mid = scores.len() / 2;
    let incomplete_score = scores[mid];

    dbg!(incomplete_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_valid() {
        let input = "(())";
        let lexed = lex(input);
        let parsed = parse(&lexed);
        assert_eq!(parsed, LineStatus::Valid)
    }

    #[test]
    fn simple_corrupt() {
        let input = "(()]";
        let lexed = lex(input);
        let parsed = parse(&lexed);
        assert_eq!(
            parsed,
            LineStatus::Corrupt {
                expected: Token::CloseParen,
                illegal: Token::CloseSquare
            }
        )
    }

    #[test]
    fn more_complex_corrupt() {
        let input = "(()[]{}<><<>>}";
        let lexed = lex(input);
        let parsed = parse(&lexed);
        assert_eq!(
            parsed,
            LineStatus::Corrupt {
                expected: Token::CloseParen,
                illegal: Token::CloseCurly
            }
        )
    }

    #[test]
    fn simple_incomplete() {
        let input = "(()";
        let lexed = lex(input);
        let parsed = parse(&lexed);
        assert_eq!(
            parsed,
            LineStatus::Incomplete {
                expected: vec![Token::CloseParen],
            }
        )
    }

    #[test]
    fn more_complex_incomplete() {
        let input = "(()[<(";
        let lexed = lex(input);
        let parsed = parse(&lexed);
        assert_eq!(
            parsed,
            LineStatus::Incomplete {
                expected: vec![
                    Token::CloseParen,
                    Token::CloseAngle,
                    Token::CloseSquare,
                    Token::CloseParen
                ],
            }
        )
    }
}
