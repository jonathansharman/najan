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
				self.expand_najan_xlit(ch);
				self.expand_najan(ch);
			}
		});
		Ok(book)
	}

	fn supports_renderer(&self, renderer: &str) -> bool {
		renderer != "not-supported"
	}
}
