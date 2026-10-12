# Hands-On 1 — CPC 2026/27

Statement: <https://pages.di.unipi.it/rossano/blog/2024/handson12425/>

**Plan:** first you solve it **by hand**. Only afterwards do we correct it together with Claude.
`src/lib.rs` contains only the prof's code: `Node`, `Tree`, `add_node`, `sum`/`rec_sum`, `test_sum`.
Nothing else: no solutions, no hints.

## Exercises (from the page)
1. A method that checks whether the binary tree is a **Binary Search Tree**.
2. **Maximum Path Sum**: return the sum of the maximum simple path connecting **two leaves**.

The page does not define the edge cases. Decide them yourself, then write them down in the report and cover them in the tests.

## Delivery (by email to rossano.venturini@gmail.com)
- `lib.rs` with the implementations and **a large set of tests**
- `Handson_1_solution_YOUR_NAME.pdf`: a brief report describing the implementations
- Before submitting: all tests pass, then `cargo fmt` and `cargo clippy`

```
cargo test   -p handson1
cargo fmt    -p handson1
cargo clippy -p handson1
```

## Collaboration policy
You may **discuss** with other students. Every implementation must be your own: sharing code is **forbidden**.
⚠️ Do not add this folder to `pubblica.json`.

## Deadline
The page gives no deadline. Check the course page / Teams (last year it was ~2 weeks).

## Note on the provided code (checked: it compiles, `test_sum` passes)
Even before you write anything, `cargo clippy` already reports some warnings on the prof's code:
- `binary comparison to literal Option::None` in `add_node`: clippy suggests `.is_none()`;
- dead code: `Tree` and `Node` are not `pub` and are used only by the tests.
Decide whether to fix them before submitting: the prof asks you to run clippy.
