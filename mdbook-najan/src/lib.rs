use mdbook::{
	book::{Book, Chapter},
	errors::Error,
	preprocess::{Preprocessor, PreprocessorContext},
	BookItem,
};
use regex::Regex;

pub struct Najan {
	najan_xlit_regex: Regex,
	najan_regex: Regex,
}

impl Najan {
	pub fn new() -> Najan {
		let najan_xlit_regex = Regex::new(r#"\{([^{}]*)}"#).unwrap();
		let najan_regex = Regex::new(r#"\{\{([^{}]*)}}"#).unwrap();
		Najan {
			najan_xlit_regex,
			najan_regex,
		}
	}

	fn expand_najan_xlit(&self, ch: &mut Chapter) {
		ch.content = self
			.najan_xlit_regex
			.replace_all(&ch.content, "<naj>${1}</naj> ⟨${1}⟩")
			.to_string();
	}

	fn expand_najan(&self, ch: &mut Chapter) {
		ch.content = self
			.najan_regex
			.replace_all(&ch.content, "<naj>${1}</naj>")
			.to_string();
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
	let mut colspan = 1;
	for row in gloss[..gloss.len() - 1].iter() {
		colspan = colspan.max(row.len());
		result.push_str(&generate_gloss_row(row));
	}
	// The last line is assumed to be a single full sentence, to be quoted.
	result.push_str(&format!(
		r#"<tr><td colspan="{colspan}">“{}”</td></tr></table>"#,
		gloss.last().unwrap(),
	));
	result
}

fn generate_gloss_row(row: &str) -> String {
	format!(
		"<tr>{}</tr>",
		row.split('|')
			.map(|word| format!("<td>{}</td>", word.trim()))
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
				self.expand_najan_xlit(ch);
			}
		});
		Ok(book)
	}

	fn supports_renderer(&self, renderer: &str) -> bool {
		renderer != "not-supported"
	}
}
