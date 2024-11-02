/// Returns an iterator over the valid Najan words within `text`. The text may
/// include whitespace between words or be in scriptio continua. If the text
/// contains invalid characters or phonotactic violations, the resulting
/// iterator may produce invalid words near the error but should recover.
pub(crate) fn lex(text: &str) -> Lexer<'_> {
	Lexer { text }
}

pub(crate) struct Lexer<'a> {
	text: &'a str,
}

enum State {
	Onset,
	MidCentralNucleus,
	PeripheralNucleus,
	Coda,
	Invalid,
}

impl<'a> Iterator for Lexer<'a> {
	type Item = &'a str;

	fn next(&mut self) -> Option<Self::Item> {
		// Ignore leading whitespace.
		self.text = self.text.trim_start();

		let mut state = State::Onset;
		let mut char_indices = self.text.char_indices().peekable();
		while let Some((i, c)) = char_indices.next() {
			let token = Token::from(c);
			match state {
				State::Onset => match token {
					Token::Invalid => state = State::Invalid,
					Token::PeripheralVowel => {
						state = State::PeripheralNucleus;
					}
					Token::MidCentralVowel => state = State::MidCentralNucleus,
					// Anything else before the vowel must be part of the onset.
					_ => {}
				},
				State::PeripheralNucleus => {
					// A peripheral vowel must be followed by a single
					// consonant.
					if token.is_consonant() {
						state = State::Coda;
					} else {
						// Cut this invalid word before the unexpected token.
						let (word, rest) = self.text.split_at(i);
						self.text = rest;
						return Some(word);
					}
				}
				State::MidCentralNucleus => {
					// This token is the coda of the current word only if it's a
					// terminal that doesn't belong to the following word.
					if let Token::Terminal = token {
						// If the next token is a vowel, then this terminal must
						// belong to the next word since a vowel is not a valid
						// onset.
						let next_is_vowel =
							char_indices.peek().map_or(false, |(_, next_c)| {
								Token::from(*next_c).is_vowel()
							});
						if !next_is_vowel {
							state = State::Coda;
							continue;
						}
					}
					// This token belongs to the next word.
					let (word, rest) = self.text.split_at(i);
					self.text = rest;
					return Some(word);
				}
				State::Coda => {
					let (word, rest) = self.text.split_at(i);
					self.text = rest;
					return Some(word);
				}
				State::Invalid => {
					// Group runs of invalid characters into a single word.
					if token != Token::Invalid {
						let (word, rest) = self.text.split_at(i);
						self.text = rest;
						return Some(word);
					}
				}
			}
		}

		// Return any remaining text.
		let word = self.text;
		self.text = "";
		(!word.is_empty()).then_some(word)
	}
}

#[derive(PartialEq, Eq)]
enum Token {
	Invalid,
	PeripheralVowel,
	MidCentralVowel,
	Terminal,
	Other,
}

impl From<char> for Token {
	fn from(c: char) -> Token {
		match c {
			'a' | 'e' | 'i' | 'u' | 'o' => Token::PeripheralVowel,
			'ʊ' | 'ɪ' | 'ə' => Token::MidCentralVowel,
			'm' | 'n' | 'ŋ' | 'l' | 'x' | 'r' | 'y' | 'w' => Token::Terminal,
			'p' | 'b' | 't' | 'd' | 'k' | 'g' | 'f' | 'v' | 'θ' | 'ð' | 's'
			| 'z' | 'c' | 'j' | 'h' | 'q' => Token::Other,
			_ => Token::Invalid,
		}
	}
}

impl Token {
	fn is_vowel(&self) -> bool {
		matches!(self, Token::PeripheralVowel | Token::MidCentralVowel)
	}

	fn is_consonant(&self) -> bool {
		matches!(self, Token::Terminal | Token::Other)
	}
}

#[cfg(test)]
mod tests {
	use rand::{seq::IteratorRandom, thread_rng};

	use super::*;

	// A random consonant.
	fn c() -> char {
		"mnŋlxrywpbtdkgfvθðszcjhq"
			.chars()
			.choose(&mut thread_rng())
			.unwrap()
	}

	// A random terminal consonant.
	fn t() -> char {
		"mnŋlxryw".chars().choose(&mut thread_rng()).unwrap()
	}

	// A random double consonant cluster.
	fn cc() -> String {
		// This isn't totally phonotactically valid, but it at least ensures
		// that the first consonant isn't terminal, which is what's important to
		// the lexer.
		let nt = "pbtdkgfvθðszcjhq"
			.chars()
			.choose(&mut thread_rng())
			.unwrap();
		format!("{}{}", nt, t())
	}

	// A random peripheral vowel.
	fn p() -> char {
		"aeiuo".chars().choose(&mut thread_rng()).unwrap()
	}

	// A random mid-central vowel.
	fn m() -> char {
		"ʊɪə".chars().choose(&mut thread_rng()).unwrap()
	}

	fn cm() -> String {
		format!("{}{}", c(), m())
	}

	fn cmt() -> String {
		format!("{}{}{}", c(), m(), t())
	}

	fn ccm() -> String {
		format!("{}{}", cc(), m())
	}

	fn ccmt() -> String {
		format!("{}{}{}", cc(), m(), t())
	}

	fn cpc() -> String {
		format!("{}{}{}", c(), p(), c())
	}

	fn ccpc() -> String {
		format!("{}{}{}", cc(), p(), c())
	}

	fn all() -> impl Iterator<Item = String> {
		[cm(), cmt(), ccm(), ccmt(), cpc(), ccpc()].into_iter()
	}

	/// Confirm that all possible combinations of word forms can be correctly
	/// lexed even without spaces between them.
	#[test]
	fn scriptio_continua() {
		for first in all() {
			for second in all() {
				let concatenation = format!("{first}{second}");
				let words: Vec<_> = lex(&concatenation).collect();
				assert_eq!(words, vec![&first, &second]);
			}
		}
	}
}
