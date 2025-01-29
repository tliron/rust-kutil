/// Value from 0 to 1000.
///
/// Used for both quality and priority.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct QValue(pub u16);

/// Max QValue (1000).
pub const MAX_QVALUE: QValue = QValue(1000);

impl QValue {
    /// Parse according to
    /// [IETF RFC 7231 section 5.3.1](https://datatracker.ietf.org/doc/html/rfc7231#section-5.3.1).
    pub fn parse(representation: &str) -> Option<Self> {
        // Based on:
        // https://github.com/tower-rs/tower-http/blob/main/tower-http/src/content_encoding.rs

        let mut chars = representation.chars();

        // Parse "q=" (case-insensitively).
        match chars.next() {
            Some('q' | 'Q') => (),
            _ => return None,
        };

        match chars.next() {
            Some('=') => (),
            _ => return None,
        };

        // Parse leading digit. Since valid q-values are between 0.000 and 1.000, only "0" and "1"
        // are allowed.
        let mut value = match chars.next() {
            Some('0') => 0,
            Some('1') => 1000,
            _ => return None,
        };

        // Parse optional decimal point.
        match chars.next() {
            Some('.') => (),
            None => return Some(Self(value)),
            _ => return None,
        };

        // Parse optional fractional digits. The value of each digit is multiplied by `factor`.
        // Since the q-value is represented as an integer between 0 and 1000, `factor` is `100` for
        // the first digit, `10` for the next, and `1` for the digit after that.
        let mut factor = 100;
        loop {
            match chars.next() {
                Some(n @ '0'..='9') => {
                    // If `factor` is less than `1`, three digits have already been parsed. A
                    // q-value having more than 3 fractional digits is invalid.
                    if factor < 1 {
                        return None;
                    }

                    // Add the digit's value multiplied by `factor` to `value`.
                    value += factor * (n as u16 - '0' as u16);
                }

                None => {
                    // No more characters to parse. Check that the value representing the q-value is
                    // in the valid range.
                    return if value <= 1000 { Some(Self(value)) } else { None };
                }

                _ => return None,
            };

            factor /= 10;
        }
    }
}
