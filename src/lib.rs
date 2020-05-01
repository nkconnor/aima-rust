//! # `aima-rust` &emsp; ![Build] ![Crate]
//!
//! [Build]: https://github.com/nkconnor/aima-rust/workflows/build/badge.svg
//! [Crate]: https://img.shields.io/crates/v/ent
//!
//! Rust implementation of programs in [**Artifical Intelligence: A Modern Approach**](http://aima.cs.berkeley.edu/).
//! For more complete examples please see the [Java](http://j) and [Python](http://p) repositories.
//! Rust is a systems programming language that aims to be [fast, reliable, and productive](https://www.rust-lang.org/),
//! and is already being used in major initiatives like HuggingFace's
//! [state of the art NLP tokenizers](https://github.com/huggingface/tokenizers) and Argo AI's [autonomous driving platform](https://www.argo.ai/).
//! It has many features that make it an attractive choice for developing AI systems. One example is
//! the basic safety guarantees provided:
//!
//! 1) Programs that compile in standard Rust are guaranteed to be free of memory corruption. If you weren't aware,
//! memory-corruption results in **undefined program behaviour**.
//! 2) Concurrent mutable access _does not even compile_. This in most cases eliminates concerns of "data races".
//!
//! Furthermore, higher-level abstractions in the language make it possible to develop similar ad-hoc safety guarantees for
//! your own programs.
//!
//! ## Getting Started
//!
//! If you are not already familiar with Rust, this may not be the best starting point. There is an
//! [excellent introductory book here that you can get started with](https://docs.rust-lang.org) and
//! use alongside this repository.
//!
//!
//! _To be filled out once these details are determined_
//!
//! ## License
//!
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//!
//!
//! ## Contribute
//!
//! Whether you are just getting started in Rust and AI, or a complete veteran, we would be happy for you to help with project.
//! Note that unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `aima-rust`
//! by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
#![doc(html_playground_url = "https://play.rust-lang.org/")]

pub mod agents;