# Najan

This repo contains resources related to the Najan
[conlang](https://en.wikipedia.org/wiki/Constructed_language).

An [mdBook](https://rust-lang.github.io/mdBook/) project
([book.toml](./book.toml) and [src/](./src/)) provides an overview of the
grammar. The [mdbook-najan/](./mdbook-najan/) crate provides a preprocessor for
conveniently inserting Najan text and translations into the book. GitHub Actions
builds and publishes the book via GitHub Pages at
https://jonathansharman.github.io/najan/.

A [`lexi`](https://github.com/jonathansharman/lexi) lexicon
([`lexicon.yaml`](./lexicon.yaml)) can be used to generate a Najan dictionary.

A [FontForge](https://fontforge.org/) project ([najan.sfd](./najan.sfd))
produces a custom font for Najan orthography, used in both the grammar book and
dictionary.

A [`tlek`](https://github.com/jonathansharman/tlek) grammar
([`phonotactics.ron`](./phonotactics.ron)) can be used to generate random valid
Najan words.
