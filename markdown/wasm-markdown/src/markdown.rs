/// Parse markdown and return html string
pub fn parse(source: String) -> String {
    Parser {
        pos: 0,
        input: source,
    }.parse_lines()
}

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_lines(&mut self) -> String {
        let mut result = String::new();

        loop {
            self.consume_whitespace();
            if self.eof() {
                break;
            }

            result.push_str(&self.parse_line());
        }

        result
    }

    /// Parse a line
    fn parse_line(&mut self) -> String {
        match self.next_char() {
            '#' => {
                self.parse_title()
            }
            '-' => {
                if char::is_whitespace(self.input[self.pos+1..].chars().next().unwrap()) {
                    self.parse_list()
                } else {
                    self.parse_text()
                }
            }
            _ => {
                self.parse_text()
            }
        }
    }

    /// Parse title
    fn parse_title(&mut self) -> String {
        let sharps = self.consume_while(|c| c == '#');
        self.consume_whitespace();
        println!("pos: {}, input: {}", self.pos, self.input);
        let text = self.parse_text();
        println!("Parsed text: {}", text);

        create_html_element(format!("h{}", sharps.len()), text)
    }

    /// Parse text
    fn parse_text(&mut self) -> String {
        self.consume_while(|c| !is_newline(c))
    }

    fn parse_list(&mut self) -> String {
        self.consume_char();
        self.consume_whitespace();

        let text = self.parse_text();
        create_html_element("li".to_string(), text)
    }

    /// Consume and discard zero or more whitespace chars
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    /// Consume characters until cond returns false
    fn consume_while<F>(&mut self, cond: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && cond(self.next_char()) {
            result.push(self.consume_char());
        }

        result
    }

    /// Return current char and advance self.pos by 1
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Peek char at self.pos
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Check if current char at self.pos starts with given char
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// check if all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

fn is_newline(c: char) -> bool {
    c == '\n'
}

fn create_html_element(tag_name: String, text: String) -> String {
    format!("<{}>{}</{}>", tag_name, text, tag_name)
}

#[cfg(test)]
mod test {
    use markdown::*;
    #[test]
    fn test_parse_lines() {
        let mut parser = Parser {
            pos: 0,
            input: "# Hello, title\nThis is takamichi\n- list\n- list".to_string(),
        };

        assert_eq!(parser.parse_lines(), "<h1>Hello, title</h1>This is takamichi<li>list</li><li>list</li>");
    }

    #[test]
    fn test_parse_line() {
        let mut parser = Parser {
            input: "# Hello, title\n".to_string(),
            pos: 0,
        };

        assert_eq!(parser.parse_line(), "<h1>Hello, title</h1>");
    }

    #[test]
    fn test_parse_title() {
        let mut parser = Parser {
            input: "# Hello, title\n".to_string(),
            pos: 0,
        };
        assert_eq!(parser.parse_title(), "<h1>Hello, title</h1>");


        let mut parser2 = Parser {
            input: "### Hello, title\n".to_string(),
            pos: 0,
        };
        assert_eq!(parser2.parse_title(), "<h3>Hello, title</h3>");
    }

    #[test]
    fn test_parse_list() {
        let mut parser = Parser {
            input: "- list item".to_string(),
            pos: 0,
        };
        assert_eq!(parser.parse_list(), "<li>list item</li>");
    }
}
