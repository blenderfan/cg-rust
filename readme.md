
## Table of contents
- [Introduction](#introduction)
- [Overview](#overview)
- [Dependencies](#dependencies)
- [Examples](#examples)

## <a id="introduction"></a>Introduction

CG Rust is an abbreviation for "**C**omputational **G**eometry for **Rust**" and is intended
to become an library full of spatial data structures and algorithms for them e.g. cutting polygons,
finding nearest neighbors in a KD-Tree etc.

However, at the moment, the library (and my rust skills) are at very beginnings and I am publishing it very early
to keep myself motivated ^^

## <a id="overview"></a>Overview

Right now, only a very basic SIMD Vector classes and rudimentary Polygons are included.

## <a id="dependencies"></a>Dependencies

To build the crate, nightly rust is necessary for using the following modules:

> [SIMD](https://doc.rust-lang.org/std/simd/index.html)

There are also dependencies for running the examples provided:

> [Plotters](https://github.com/plotters-rs/plotters)

## <a id="examples"></a>Examples

![Plotter Polygon Example](https://github.com/blenderfan/cg-rust/blob/master/examples/plotters/plot-data/polygon.png)

To run the examples in the crate that use plotters, use:

`cargo run --example plotter_[example_name]`

E.g. `cargo run --example plotter_polygon`