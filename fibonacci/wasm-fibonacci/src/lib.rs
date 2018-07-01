#[no_mangle]
pub extern "C" fn fib(v: usize) -> usize {
    if v == 1 {
        return 1
    }
    if v == 2 {
        return 1
    }

    fib(v - 1) + fib(v - 2)
}

