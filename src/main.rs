#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Heading(HeadingLevel, String)
}


#[derive(Clone, Debug, PartialEq)]
pub enum HeadingLevel {
    H1 = 1,
}

fn heading_level_count(heading: &'static str) -> usize {
    let mut chars = heading.chars().peekable();

    let mut level = 0;

    while chars.peek() == Some(&'#') {
        chars.next();
        level += 1;
    }

    level

}

fn main() {
    println!("level = {}", heading_level_count("###"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_level_count_test() {
        assert_eq!(heading_level_count("#"), 1);
        assert_eq!(heading_level_count("##"), 2);
        assert_eq!(heading_level_count("###HelloWorld"), 3);
        assert_eq!(heading_level_count("### ####"), 3);
        assert_eq!(heading_level_count("Hello World"), 0);
    }
}
