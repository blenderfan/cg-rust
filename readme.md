
## Table of contents
- [Introduction](#introduction)
- [Overview](#overview)
- [Features](#features)
- [Examples](#examples)
- [Dependencies](#dependencies)

## <a id="introduction"></a>Introduction

CG Rust is an abbreviation for "**C**omputational **G**eometry for **Rust**" and is intended
to become an library full of spatial data structures and algorithms for them e.g. cutting polygons,
finding nearest neighbors in a KD-Tree etc.

However, at the moment, the library (and my rust skills) are at very beginnings and I am publishing it very early
to keep myself motivated ^^

## <a id="overview"></a>Overview

A basic SIMD Vector class is included as well as rudimentary Polygons. As of now, they can be fan triangulated, can be checked
if they are convex or convave. It is possible to use them for very simple rendering tasks (see examples)

## <a id="features"></a>Features

### C Export

CG Rust can be compiled as a C-compatible library, exposing most of the functionality to other languages. Note, that for generic functions, not all possibilities are exported, but usually only single- and double-precision versions (which can then for example be used in [Unity](https://github.com/blenderfan/CGRust-UnityWrapper))

In order to achieve this, it is required to install [cbindgen](https://github.com/mozilla/cbindgen) first, using the following command in a shell:

`cargo install --force cbindgen`

The simplest way to generate the headers and the library is to execute the powershell script called **cbindgen.ps1** in the top folder.
Otherwise, if you want to do it on your own, then whenever you want to make the library compatible with C, the feature **"c_export"** should be activated. Here an example for how the command could look like:

`cargo rustc --crate-type=cdylib`

The resulting DLL can be integrated into other projects in C#, [Python](https://docs.python.org/3/library/ctypes.html), etc. Afterwards, the C++-Header has to be automatically generated from the Rust Files. To generate, simply execute the following command:

`cbindgen --config cbindgen.toml --crate cg-rust --output autogen/cg_rust.h`

## <a id="examples"></a>Examples

![Plotter Polygon Example](https://github.com/blenderfan/cg-rust/blob/master/examples/plotters/plot-data/polygon.png)
![WGPU Polygon Example](https://github.com/blenderfan/cg-rust/blob/master/examples/wgpu/polygon/polygon.jpg)

To run the examples in the crate that use plotters, use:

`cargo run --example plotter_[example_name]`

E.g. `cargo run --example plotter_polygon`

To run examples that use WGPU, use:

`cargo run --example wgpu_[example_name]`

E.g. `cargo run --example wgpu_polygon`

## <a id="dependencies"></a>Dependencies

To build the crate, nightly rust is necessary for using the following modules:

> [SIMD](https://doc.rust-lang.org/std/simd/index.html)

If the **c_export** feature is used when building, the following dependency is required:

> [libc](https://github.com/rust-lang/libc)

Additionally, the following dependencies exist:

> [num_traits](https://docs.rs/num-traits/latest/num_traits/index.html)

There are additional dependencies for running the examples provided:

> [Plotters](https://github.com/plotters-rs/plotters)
> [WGPU](https://github.com/gfx-rs/wgpu)

