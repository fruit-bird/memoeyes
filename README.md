# MemoEYES 👁️

Two procedual macros for automatically implementing memoization for your functions, making recursive function calls ***FAST***

Might update this sometime to merge both macros into a single one with different args

## `#[lru_cache]`
This macro creates a global static variable and uses it for memoization. It's also an LRU cache which makes it more convenient
```rust
#[lru_cache(max = 10)]
fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

let result = fib(186);
// result: 332825110087067562321196029789634457848
```

## `#[memo]`
This macro is more explicit (which better follows Rust's philosophy) and does not use unsafe code. It modifies the function so that it has an extra argument that's a `HashMap<TUPLE_OF_INPUT_TYPES, OUTPUT_TYPE>`

Using this allows you to directly access the lookup table without having to go through unsafe blocks and implicit code
```rust
#[memo]
fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

let mut memo = HashMap::new();
let result = fib(186, &mut memo);
// result: 332825110087067562321196029789634457848
```

## Contribution
Please do

## What is This?
Wanted to get more familiar with function memoization, and wanted to learn attribute-like macros. Worked itself out. Unimportant but [name inspiration](https://www.youtube.com/watch?v=CQCLMpyf66A)
