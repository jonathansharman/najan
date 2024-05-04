use std::collections::HashMap;

use lexi::lexicon::{Lexeme, Lexicon};
use mdbook::{
	book::{Book, Chapter},
	errors::Error,
	preprocess::{Preprocessor, PreprocessorContext},
	BookItem,
};
use regex::Regex;

pub struct Najan {
	najan_regex: Regex,
	lexemes_by_lemma: HashMap<String, Lexeme>,
}

impl Najan {
	pub fn new() -> Najan {
		// Compile regex for custom markup.
		let najan_regex = Regex::new(r"\{([^{}]*)}").unwrap();

		// Get lexemes indexed by lemma.
		let lexicon_bytes = include_str!("../../lexicon.yaml").as_bytes();
		let lexicon = Lexicon::open(lexicon_bytes).unwrap();
		let lexemes_by_lemma = lexicon
			.lexemes
			.into_iter()
			.map(|lexeme| (lexeme.lemma.clone(), lexeme))
			.collect();

		Najan {
			najan_regex,
			lexemes_by_lemma,
		}
	}

	fn expand_najan(&self, ch: &mut Chapter) {
		ch.content = self
			.najan_regex
			.replace_all(&ch.content, |captures: &regex::Captures| -> String {
				captures[1]
					.split_whitespace()
					.map(|word| self.expand_najan_word(word))
					.collect::<Vec<_>>()
					.join(" ")
			})
			.to_string();
	}

	fn expand_najan_word(&self, word: &str) -> String {
		match self.lexemes_by_lemma.get(word) {
			Some(lexeme) => {
				let glosses = if lexeme.glosses.is_empty() {
					String::new()
				} else {
					format!(" — {}", lexeme.glosses.join("; "))
				};
				let translation = lexeme
					.translation
					.as_ref()
					.map(|translation| format!("<br>{translation}"))
					.unwrap_or_default();
				format!(
					r#"<span class="najan-tooltip"><span class="najan"><a href="/dictionary.html#{word}" target="_blank">{word}</a></span><span class="najan-tooltip-text"><span class="najan">{word}</span> ⟨{word}⟩{glosses}{translation}</span></span>"#
				)
			}
			None => format!(
				r#"<span class="najan" style="text-decoration: wavy red underline">{word}</span>"#
			),
		}
	}

	fn expand_interlinear_gloss(&self, ch: &mut Chapter) {
		let mut new_content = Vec::new();
		let mut in_gloss = false;
		let mut gloss = Vec::new();
		for line in ch.content.lines() {
			if line == "<gloss>" {
				in_gloss = true;
			} else if in_gloss {
				if line == "</gloss>" {
					new_content.push(generate_gloss(&gloss));
					gloss.clear();
					in_gloss = false;
				} else {
					gloss.push(line.to_owned());
				}
			} else {
				new_content.push(line.to_owned());
			}
		}
		ch.content = new_content.join("\n");
	}
}

fn generate_gloss(gloss: &[String]) -> String {
	let mut result = "<table class='gloss'>".to_string();
	// The first line is assumed to be a raw Najan transcription. Output it
	// first in Najan script (by wrapping in {}) and then italicized.
	let mut colspan = gloss[0].len();
	result.push_str(&generate_gloss_row(&gloss[0], "{", "}"));
	result.push_str(&generate_gloss_row(&gloss[0], "<i>", "</i>"));
	// Middle rows should be glosses, which can be output as-is.
	for row in gloss[1..gloss.len() - 1].iter() {
		colspan = colspan.max(row.len());
		result.push_str(&generate_gloss_row(row, "", ""));
	}
	// The last line is assumed to be a single full sentence, to be quoted.
	result.push_str(&format!(
		r#"<tr><td colspan="{colspan}">“{}”</td></tr></table>"#,
		gloss.last().unwrap(),
	));
	result
}

fn generate_gloss_row(row: &str, left: &str, right: &str) -> String {
	format!(
		"<tr>{}</tr>",
		row.split('|')
			.map(|word| format!("<td>{left}{}{right}</td>", word.trim()))
			.collect::<Vec<String>>()
			.join("")
	)
}

impl Default for Najan {
	fn default() -> Self {
		Self::new()
	}
}

impl Preprocessor for Najan {
	fn name(&self) -> &str {
		"najan-preprocessor"
	}

	fn run(
		&self,
		_ctx: &PreprocessorContext,
		mut book: Book,
	) -> Result<Book, Error> {
		book.for_each_mut(|item| {
			if let BookItem::Chapter(ch) = item {
				self.expand_interlinear_gloss(ch);
				self.expand_najan(ch);
			}
		});
		Ok(book)
	}

	fn supports_renderer(&self, renderer: &str) -> bool {
		renderer != "not-supported"
	}
}
