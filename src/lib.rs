//! gen-nested-iter-yield exports a helper macro, nested_iter_yield, which can be used to
//! generate n-nested for loops over identical iterators. This is useful for generating a
//! stream of permutations with replacement without storing unnecessary intermediary buffers.
//!
//! The macro returns a [genawaiter::sync](https://docs.rs/genawaiter/latest/genawaiter/sync/index.html)
//! generator which, when the feature `futures03` is enabled, implements [futures::stream::Stream](https://docs.rs/futures/0.3.21/futures/stream/index.html).

extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

enum PreOrPostFix {
    Pre(String),
    Post(String),
}

/// Creates an n-nested for loop of iterators over the passed input. Assumes that the first
/// argument can be called multiple times to produce identical iterators.
///
/// ## Usage
/// ### Basic syntax
/// `nested_iter_yield!(iterable_name, 3)`
///
/// ### Dereference values from vector iterable
/// `nested_iter_yield!(vector_name.iter(), 3, *)`
///
/// The third argument is optional. Here `*` is prefixed to each result of the iterator. Any value
/// passed as the third argument that does not begin with a `.` will be prefixed to each result
/// of the iterator.
///
/// ### Clone values from vector iterable
///
/// `nested_iter_yield!(vector_name.iter(), 3, .clone())`
///
/// Here `.clone()` is appended to each result of the iterator. Any value that begins with `.` will
/// be treated as a postfix.
#[proc_macro]
pub fn nested_iter_yield(item: TokenStream) -> TokenStream {
    // parse input
    let item_string = item.to_string();
    let inps = item_string.split(",").collect::<Vec<_>>();
    let source_iter = inps[0].trim();
    let n = usize::from_str(inps[1].trim())
        .expect("nested_iter_yield: could not parse input n as usize.");
    let value_transformation = match inps.len() {
        2 => None,
        l if l > 2 => Some(match inps[2] {
            m if m.starts_with(".") => PreOrPostFix::Post(inps[2].to_string()),
            _ => PreOrPostFix::Pre(inps[2].to_string()),
        }),
        _ => unreachable!("missing input fields to nested_iter_yield macro"),
    };

    // generate code
    let generator_open = "genawaiter::sync::Gen::new(|co| async move {".to_string();
    let open_loops = (0..n)
        .map(|i| format!("for val_{i} in {source_iter} {{"))
        .collect::<Vec<_>>()
        .join("\n");
    let yield_open = "co.yield_(vec![";
    let yield_args = (0..n)
        .map(|i| format!("val_{i}"))
        .map(|variable| match &value_transformation {
            None => variable,
            Some(trans) => match trans {
                PreOrPostFix::Pre(pre) => pre.to_string() + variable.as_str(),
                PreOrPostFix::Post(post) => variable + post.as_str(),
            },
        })
        .collect::<Vec<_>>()
        .join(", ");
    let yield_close = "]).await;";
    let close_loops = (0..n).map(|_| "}").collect::<Vec<_>>().join("\n");
    let generator_closed = "})";
    let generated_code = generator_open
        + open_loops.as_str()
        + yield_open
        + yield_args.as_str()
        + yield_close
        + close_loops.as_str()
        + generator_closed;

    // convert to TokenStream and return
    generated_code.parse().unwrap()
}
