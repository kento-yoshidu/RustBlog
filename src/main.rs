use std::{arch::x86_64::_CMP_TRUE_UQ, fmt::format};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Token {
    Heading(HeadingLevel, String),
    Bold(String),
    Text(String),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

fn tokenize(heading: &'static str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut text_buffer = String::new();
    let mut in_bold = false;

    let mut line = heading.chars().peekable();

    while let Some(c) = line.next() {
        // 行の1文字目
        match (c, in_bold) {
            ('#', false) => {
                // h要素
                let mut heading_level = 1;

                while line.peek() == Some(&'#') {
                    line.next();
                    heading_level += 1;
                }

                while let Some(' ') = line.peek() {
                    line.next();
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

                    line.clone().collect()
                ));

                break;
            },
            ('*', false) if line.peek() == Some(&'*') => {
                line.next();

                if !text_buffer.is_empty() {
                    tokens.push(Token::Text(text_buffer.clone()));
                    text_buffer.clear();
                }

                in_bold = true;
            },
            ('*', true) if line.peek() == Some(&'*') => {
                line.next();

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

    tokens
}

fn main() {
    let h = "## Hello World";
    let p = "Hello World";
    let b = "wow **Hello World** foo";

    println!("{:?}", tokenize("**hello"));
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
