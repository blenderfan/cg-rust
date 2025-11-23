
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

A basic SIMD Vector class is included as well as rudimentary Polygons. As of now, they can be fan triangulated, can be checked
if they are convex or convave. It is possible to use them for very simple rendering tasks (see examples)

## <a id="dependencies"></a>Dependencies

To build the crate, nightly rust is necessary for using the following modules:

> [SIMD](https://doc.rust-lang.org/std/simd/index.html)

Additionally, the following dependencies exist:

> [num_traits](https://docs.rs/num-traits/latest/num_traits/index.html)

There are additional dependencies for running the examples provided:

> [Plotters](https://github.com/plotters-rs/plotters)
> [WGPU](https://github.com/gfx-rs/wgpu)

## <a id="examples"></a>Examples

![Plotter Polygon Example](https://github.com/blenderfan/cg-rust/blob/master/examples/plotters/plot-data/polygon.png)
![WGPU Polygon Example](https://github.com/blenderfan/cg-rust/blob/master/examples/wgpu/polygon/polygon.jpg)

To run the examples in the crate that use plotters, use:

`cargo run --example plotter_[example_name]`

E.g. `cargo run --example plotter_polygon`

To run examples that use WGPU, use:

`cargo run --example wgpu_[example_name]`

E.g. `cargo run --example wgpu_polygon`