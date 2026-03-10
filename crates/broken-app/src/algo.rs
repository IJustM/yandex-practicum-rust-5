use std::collections::HashSet;

/// Более быстрая реализация slow_dedup
pub fn dedup(values: &[u64]) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    let mut seen = HashSet::new();
    for v in values {
        if seen.insert(v) {
            out.push(*v);
        }
    }
    out.sort();
    out
}

/// Более быстрая реализация slow_fib
pub fn fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
