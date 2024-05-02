mod token;
mod ast;
mod parse;

use parse::parse;
use token::{Token, HeadingLevel};

fn tokenize(input: &'static str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut text_buffer = String::new();
    let mut in_bold = false;

    for line in input.lines() {
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            // 行の1文字目
            match (c, in_bold) {
                ('#', false) => {
                    // h要素
                    let mut heading_level = 1;

                    while chars.peek() == Some(&'#') {
                        chars.next();
                        heading_level += 1;
                    }

                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }

                    tokens.push(Token::Heading(
                        match heading_level {
                            1 => HeadingLevel::H1,
                            2 => HeadingLevel::H2,
                            3 => HeadingLevel::H3,
                            4 => HeadingLevel::H4,
                            5 => HeadingLevel::H5,
                            6 => HeadingLevel::H6,
                            _ => unreachable!(),
                        },

                        chars.clone().collect()
                    ));

                    break;
                },
                ('*', false) if chars.peek() == Some(&'*') => {
                    chars.next();

                    if !text_buffer.is_empty() {
                        tokens.push(Token::Text(text_buffer.clone()));
                        text_buffer.clear();
                    }

                    in_bold = true;
                },
                ('*', true) if chars.peek() == Some(&'*') => {
                    chars.next();

                    tokens.push(Token::Bold(text_buffer.clone()));
                    text_buffer.clear();

                    in_bold = false;
                },
                _ => {
                    // p要素
                    text_buffer.push(c)
                }
            }
        }

    // **が閉じられない時
    if in_bold {
        tokens.push(Token::Text(format!("**{}", text_buffer)));
        text_buffer.clear();
        in_bold = false;
    }

    if !text_buffer.is_empty() {
        tokens.push(Token::Text(text_buffer.clone()));
        text_buffer.clear();
    }

    tokens.push(Token::Text("\n".to_string()));

    if let Some(Token::Text(last)) = tokens.last() {
        if last == "\n" {
            tokens.pop();
        }
    }
    }

    tokens
}

fn main() {
    let tokens = tokenize("## hello world \n learning Rust.");

    parse(&tokens);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        // h要素
        assert_eq!(tokenize("# hello"), vec![Token::Heading(HeadingLevel::H1, "hello".to_string())]);
        assert_eq!(tokenize("## hello world"), vec![Token::Heading(HeadingLevel::H2, "hello world".to_string())]);
        assert_eq!(tokenize("### hello world"), vec![Token::Heading(HeadingLevel::H3, "hello world".to_string())]);
        assert_eq!(tokenize("#### hello world"), vec![Token::Heading(HeadingLevel::H4, "hello world".to_string())]);
        assert_eq!(tokenize("##### hello world"), vec![Token::Heading(HeadingLevel::H5, "hello world".to_string())]);
        assert_eq!(tokenize("###### hello world"), vec![Token::Heading(HeadingLevel::H6, "hello world".to_string())]);

        // bold
        assert_eq!(tokenize("**hello world**"), vec![Token::Bold(("hello world").to_string())]);
        assert_eq!(tokenize("my **Rust** blog"), vec![Token::Text("my ".to_string()), Token::Bold(("Rust").to_string()), Token::Text(" blog".to_string())]);
        assert_eq!(tokenize("**hello world"), vec![Token::Text("**hello world".to_string())]);
        // assert_eq!(tokenize("my **Rust blog"), vec![Token::Text("**hello world".to_string())]);

        // p要素
        assert_eq!(tokenize("hello world"), vec![Token::Text("hello world".to_string())]);
    }
}
