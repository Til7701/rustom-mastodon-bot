use crate::io;

pub struct PatternParser {
    pub words_path: String,
}

impl PatternParser {
    pub fn parse_pattern(&self, pattern: &str) -> String {
        let mut sentence: String = String::from("");
        let mut i: usize = 0;

        while i < pattern.len() {
            let c = pattern.chars().nth(i).unwrap().to_string();
            if c == "\\" {
                // skip, if the next character is escaped
                let next_c = pattern.chars().nth(i + 1).unwrap().to_string();
                sentence.push_str(&next_c);
                i = i + 1;
            } else if c == "$" {
                // interpret the element
                let end = self.get_index_next_str(pattern, i, "}");
                match self.parse_word(pattern, i + 1, end) {
                    Some(s) => sentence.push_str(s.as_str()),
                    None => panic!(),
                }
                i = end;
            } else {
                // if the character is not a $ or \, the character can always be pushed to the reuslt
                sentence.push_str(&c);
            }
            i = i + 1;
        }

        return sentence;
    }

    fn parse_word(&self, s: &str, start: usize, end: usize) -> Option<String> {
        let tmp: String = s.chars().skip(start + 1).take(end - start - 1).collect();
        print!("Parsing word: {}", &tmp);
        let mut path = String::from(self.words_path.clone());
        path.push_str(&tmp);
        return Some(io::get_random_line_of_file(&path));
    }

    fn get_index_next_str(&self, s: &str, start: usize, search: &str) -> usize {
        let mut i: usize = start;

        while i < s.len() {
            let c = s.chars().nth(i).unwrap().to_string();
            if c == search {
                return i;
            }
            i = i + 1;
        }
        return i;
    }
}
