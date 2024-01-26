use crate::io;
use crate::pattern_parser::PatternParser;

pub struct SentenceProvider {
    pub words_path: String,
    pattern_parser: PatternParser,
}

impl SentenceProvider {
    pub fn new(words_path: String) -> Self {
        Self {
            words_path: words_path.clone(),
            pattern_parser: PatternParser {
                words_path: words_path.clone(),
            },
        }
    }

    pub fn get_random_sentence(&self) -> String {
        let sentence_pattern = self.random_pattern();

        return self.pattern_parser.parse_pattern(&sentence_pattern);
    }

    fn random_pattern(&self) -> String {
        let file_path = "sentence_patterns";
        return io::get_random_line_of_file(file_path);
    }
}
