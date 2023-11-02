use memo_attribute::lru_cache;

#[lru_cache(max = 3)]
fn fib(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[test]
fn memoized_fib_test() {
    let big = fib(186);
    println!("{}", big);
}

#[allow(unused)]
mod what_it_should_expand_to {
    use lru::LruCache;
    use once_cell::sync::Lazy;
    use std::num::NonZeroUsize;

    static mut FIB_CACHE: Lazy<LruCache<u128, u128>> =
        Lazy::new(|| LruCache::new(unsafe { NonZeroUsize::new_unchecked(10) }));

    fn fib(n: u128) -> u128 {
        if n < 2 {
            return n;
        }

        let result = unsafe {
            if let Some(result) = FIB_CACHE.get(&n) {
                return *result;
            }

            let result = fib(n - 1) + fib(n - 2);
            FIB_CACHE.put(n, result);

            // SAFETY: We just inserted the value
            FIB_CACHE.get(&n).unwrap_unchecked()
        };

        *result
    }

    #[test]
    fn memoized_fib_test() {
        let big = fib(186);
        println!("{}", big);
    }
}
