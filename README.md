# Najan

This repo contains resources related to the Najan
[conlang](https://en.wikipedia.org/wiki/Constructed_language).

An [mdBook](https://rust-lang.github.io/mdBook/) project
([book.toml](./book.toml) and [src/](./src/)) provides an overview of the
language. GitHub Actions builds this book to the [docs/](./docs/) directory, and
it is served via GitHub pages at https://jonathansharman.github.io/najan/. The
[mdbook-najan/](./mdbook-najan/) crate provides an `mdBook` preprocessor for
conveniently inserting Najan text and translations.

A [FontForge](https://fontforge.org/) project ([najan.sfd](./najan.sfd))
generates a custom font for Najan orthography.
