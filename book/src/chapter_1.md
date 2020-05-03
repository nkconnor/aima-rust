# Introduction

This book is a Rust implementation of programs in [**Artifical Intelligence: A Modern Approach**](http://aima.cs.berkeley.edu/).
For more complete examples please see the [Java](http://j) and [Python](http://p) repositories.
Rust is a systems programming language that aims to be [fast, reliable, and productive](https://www.rust-lang.org/),
and is already being used in major initiatives like HuggingFace's
[state of the art NLP tokenizers](https://github.com/huggingface/tokenizers) and Argo AI's 
[autonomous driving platform](https://argo.ai). It has many features that make it an attractive choice for developing AI
systems. One example is the basic safety guarantees provided:

1) Programs that compile in standard Rust are guaranteed to be free of memory corruption. If you weren't aware,
memory-corruption results in **undefined program behaviour**.

2) Concurrent mutable access _does not even compile_. This in most cases eliminates concerns of "data races".
Furthermore, higher-level abstractions in the language make it possible to develop similar ad-hoc safety guarantees for
your own programs.

If you are not already familiar with Rust, this may not be the best starting point. There is an
[excellent introductory book here that you can get started with](https://docs.rust-lang.org) and
use alongside this repository. The documentation is gentle in the introductory sections but
quickly builds up to assume that you have at minimum beginner knowledge of programming in Rust.

_Instructions on how to use this as a companion to studying the book goes here_