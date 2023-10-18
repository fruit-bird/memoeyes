use std::collections::HashMap;

use memo_derive::memo;

#[memo]
pub fn fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    fib_memo(n - 1) + fib_memo(n - 2)
}

#[test]
fn fib_test() {
    let mut map = HashMap::new();
    map.insert((1, 2), 3);
    let big = fib_memo(100, &mut map);
    dbg!(&big);
}

// // this is the new function
// pub fn fib(args: types**) -> return_type {
//     // this is the old function
//     fn fib_internal(args: types**, map: &mut HashMap<types**, return_type>) -> return_type {
//         memo early return predicate
//         functions stmts w/o return
//         insert new calculation into memo
//         return stmt
//     }

//     let mut map = HashMap::new();
//     return fib_internal(args**, &mut map)
// }
