# gen-nested-iter-yield

[![crates.io](https://img.shields.io/crates/v/gen-nested-iter-yield.svg)](https://crates.io/crates/gen-nested-iter-yield)
[![docs.rs](https://img.shields.io/docsrs/gen-nested-iter-yield.svg)](https://docs.rs/gen-nested-iter-yield)
[![CI](https://github.com/DominicBurkart/gen-nested-iter-yield/actions/workflows/rust.yml/badge.svg)](https://github.com/DominicBurkart/gen-nested-iter-yield/actions/workflows/rust.yml)

`gen-nested-iter-yield` exports a helper macro, `nested_iter_yield`, which can be used to
generate n-nested for loops over identical iterators. This is useful for generating a
stream of permutations with replacement without storing unnecessary intermediary buffers.

The macro returns a [genawaiter::sync](https://docs.rs/genawaiter/latest/genawaiter/sync/index.html)
generator which, when the feature `futures03` is enabled, implements [futures::stream::Stream](https://docs.rs/futures/0.3.21/futures/stream/index.html).