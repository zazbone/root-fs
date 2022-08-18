# Root io

## Description
A file system explorer for root cern file format.

### Motivations
[ROOT][rootweb] is a huge monolithic c++ data analysis framework. It provides an unparalleled amount of tools for big experimental data analysis.
It's widely used in high energy physics.

The monolithic design and the old c++ code base, make it complicate to use and deploy (especially on Window).
Now, many analysis other framework or library provides tools could be used. (Apache dataframe, [Polars][polars], [Dask][dask] ...).

Some work have been done on ROOT for interoperability with some of them. 
And ROOT RDataFrame provide a modern interface for the ROOT framework.

The goal is not to create something better than ROOT, but allow the user to choose the tool they want.
As many other libraries in other languages, this crate aim to provide a way to open root file system binary format in rust without c++ dependencies.

### Alice-rs framework
An unofficial ALICE analysis [framework][alicers], was made in order to show how to use [ALICE experiment][alicexp] data in rust.

This framework provides a `root-io` module. But it presents some issues at usage, and doesn't have many features.

This crate is not related with alice-rs framework except for the inspiration.

## Docs
Install mdbook by following the [mdbook tutorial][mdbook-book]

Then execute `mdbook serve --open docs` in the project main folder to open the book in local host.

## Contributing
The project is on early design draft, the only way to provide help is by sharing resources on root cern file system specification :)


[mdbook-book]: https://rust-lang.github.io/mdBook/index.html "Online mdbook book"
[rootweb]: https://root.cern/ "ROOT home website"
[alicers]: https://github.com/cbourjau/alice-rs "ALICE analysis framework github page"
[alicexp]: https://alice.cern/ "ALICE experiment website - main"
[polars]: https://www.pola.rs/ "polars dataframe website"
[dask]: https://www.dask.org/ "Dask website"