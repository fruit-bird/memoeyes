use std::collections::HashMap;

// use memo_attribute::memo;

// #[memo]
fn auto_fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    auto_fib(n - 1) + auto_fib(n - 2)
}

// What `#[memo] auto_fib(...)` should expand into
fn manual_memo_fib(n: u128) -> u128 {
    fn fib_internal(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
        if let Some(result) = memo.get(&(n)) {
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
fn manual_fib_test() {
    let big = manual_memo_fib(186);
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
