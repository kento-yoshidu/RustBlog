#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Token {
    Heading(HeadingLevel, String)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

fn tokenize(heading: &'static str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut line = heading.chars().peekable();

    while let Some(c) = line.next() {
        // 行の1文字目
        match c {
            '#' => {
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
            },
            _ => {
                ()
            }
        }
    }

    tokens
}

fn main() {
    let str = "## Hello World";

    println!("{:?}", tokenize(str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("# hello"), vec![Token::Heading(HeadingLevel::H1, "hello".to_string())]);
        assert_eq!(tokenize("## hello world"), vec![Token::Heading(HeadingLevel::H2, "hello world".to_string())]);
        assert_eq!(tokenize("### hello world"), vec![Token::Heading(HeadingLevel::H3, "hello world".to_string())]);
        assert_eq!(tokenize("#### hello world"), vec![Token::Heading(HeadingLevel::H4, "hello world".to_string())]);
        assert_eq!(tokenize("##### hello world"), vec![Token::Heading(HeadingLevel::H5, "hello world".to_string())]);
        assert_eq!(tokenize("###### hello world"), vec![Token::Heading(HeadingLevel::H6, "hello world".to_string())]);
    }
}
