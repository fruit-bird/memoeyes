use memo_attribute::lru_cache;

#[lru_cache(max = 100)]
fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[test]
fn memoized_fib_test() {
    let _big = fib(186);
    unsafe {
        for (k, v) in FIB_CACHE.iter() {
            println!("{:>3?}: {:>39?}", k, v);
        }
    }
}
