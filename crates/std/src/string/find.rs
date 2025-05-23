//
// FindIgnoreEscaped
//

/// Find while ignoring escaped patterns.
pub trait FindIgnoreEscaped {
    /// Find while ignoring escaped patterns.
    fn find_ignore_escaped(&self, pattern: &str) -> Option<usize>;

    /// Split once while ignoring escaped patterns.
    fn split_once_ignore_escaped(&self, pattern: &str) -> Option<(&str, &str)>;
}

impl FindIgnoreEscaped for str {
    fn find_ignore_escaped(&self, pattern: &str) -> Option<usize> {
        let pattern_chars: Vec<_> = pattern.chars().collect();
        let pattern_chars_count = pattern_chars.len();

        let mut escaped = false;
        let mut pattern_chars_index = 0;

        for (byte_index, c) in self.char_indices() {
            if c == pattern_chars[pattern_chars_index] {
                pattern_chars_index += 1;
                if pattern_chars_index == pattern_chars_count {
                    if escaped {
                        pattern_chars_index = 0;
                        escaped = false;
                    } else {
                        return Some(byte_index - pattern.len() + 1);
                    }
                }
            } else if c == '\\' {
                escaped = true;
            } else {
                escaped = false;
            }
        }

        None
    }

    fn split_once_ignore_escaped(&self, pattern: &str) -> Option<(&str, &str)> {
        self.find_ignore_escaped(pattern).map(|index| (&self[..index], &self[index + pattern.len()..]))
    }
}
