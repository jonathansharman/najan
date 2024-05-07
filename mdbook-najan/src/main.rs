use anyhow::Result;
use clap::{Arg, Command};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_najan::Najan;
use semver::Version;
use semver::VersionReq;

use std::fs::File;
use std::io;

const LEXICON_ARG: &str = "lexicon";
const SUPPORTS_COMMAND: &str = "supports";

pub fn make_app() -> Command {
	Command::new("mdbook-najan")
		.about("A mdBook preprocessor to add some shortcuts for the Najan book")
		.arg(Arg::new(LEXICON_ARG).index(1).required(true))
		.subcommand(
			Command::new(SUPPORTS_COMMAND)
				.arg(Arg::new("renderer").required(true))
				.about("Check whether a renderer is supported by this preprocessor"),
		)
}

fn main() -> Result<()> {
	let matches = make_app().get_matches();

	// Support all renderers implicitly.
	if matches.subcommand_matches(SUPPORTS_COMMAND).is_some() {
		return Ok(());
	}

	let lexicon_filename = matches.get_one::<String>(LEXICON_ARG).unwrap();
	let preprocessor = Najan::new(File::open(lexicon_filename)?);
	handle_preprocessing(&preprocessor)
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<()> {
	let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

	let book_version = Version::parse(&ctx.mdbook_version)?;
	let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

	if !version_req.matches(&book_version) {
		eprintln!(
			"Warning: The {} plugin was built against version {} of mdbook, \
			but we're being called from version {}",
			pre.name(),
			mdbook::MDBOOK_VERSION,
			ctx.mdbook_version
		);
	}

	let processed_book = pre.run(&ctx, book)?;
	serde_json::to_writer(io::stdout(), &processed_book)?;

	Ok(())
}
