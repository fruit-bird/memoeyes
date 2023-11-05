use std::collections::HashMap;

use memoeyes::memo;

#[memo]
fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[test]
fn memoized_fib_test() {
    let mut memo = HashMap::new();
    let _big = fib(186, &mut memo);
    println!("{:#?}", memo);
}
