use std::collections::HashMap;

use memo_attribute::memo;

#[memo]
fn auto_fib(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    auto_fib(n - 1) + auto_fib(n - 2)
}

/// What `#[memo]` should expand into
fn manual_memo_fib(n: usize) -> usize {
    fn fib_internal(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(result) = memo.get(&n) {
            return result.clone();
        }

        if n < 2 {
            return n;
        }

        let result = fib_internal(n - 1, memo) + fib_internal(n - 2, memo);
        memo.insert(n, result);

        result
    }

    let mut map = HashMap::new();
    let result = fib_internal(n, &mut map);
    result // or map[&n]
}

#[test]
fn fib_test() {
    let big = manual_memo_fib(60);
    println!("{}", big);
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
//     let result = fib_internal(args**, &mut map);
//     result
// }
