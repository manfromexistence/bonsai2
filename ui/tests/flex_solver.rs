//! Differential test: the analytic water-filling solver against the CSS
//! freeze-and-redistribute reference algorithm.

use ui::layout::{solve_grow, solve_grow_reference, solve_shrink, solve_shrink_reference};

/// xorshift64, so the test is deterministic without a dependency.
struct Rng(u64);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 32) as u32
    }

    fn range(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (self.next_u32() % 100_000) as f32 / 100_000.0 * (hi - lo)
    }
}

struct Case {
    base: Vec<f32>,
    grow: Vec<f32>,
    shrink: Vec<f32>,
    min: Vec<f32>,
    max: Vec<f32>,
    free: f32,
    deficit: f32,
}

fn random_case(rng: &mut Rng) -> Case {
    let n = 1 + (rng.next_u32() % 12) as usize;
    let mut base = Vec::with_capacity(n);
    let mut grow = Vec::with_capacity(n);
    let mut shrink = Vec::with_capacity(n);
    let mut min = Vec::with_capacity(n);
    let mut max = Vec::with_capacity(n);
    for _ in 0..n {
        let lo = rng.range(0.0, 30.0);
        let hi = lo + rng.range(0.0, 90.0);
        let b = rng.range(0.0, 120.0).clamp(lo, hi);
        base.push(b);
        grow.push(if rng.next_u32() % 5 == 0 {
            0.0
        } else {
            rng.range(0.0, 3.0)
        });
        shrink.push(if rng.next_u32() % 5 == 0 {
            0.0
        } else {
            rng.range(0.0, 3.0)
        });
        min.push(lo);
        max.push(hi);
    }
    Case {
        base,
        grow,
        shrink,
        min,
        max,
        free: rng.range(0.0, 300.0),
        deficit: rng.range(0.0, 250.0),
    }
}

#[test]
fn analytic_grow_matches_css_reference() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for case_no in 0..5000 {
        let c = random_case(&mut rng);
        let a = solve_grow(&c.base, &c.grow, &c.min, &c.max, c.free);
        let r = solve_grow_reference(&c.base, &c.grow, &c.min, &c.max, c.free);
        for i in 0..c.base.len() {
            let tol = 1e-3 * a[i].abs().max(r[i].abs()).max(1.0);
            assert!(
                (a[i] - r[i]).abs() <= tol,
                "grow mismatch (case {case_no}, item {i}): analytic={} ref={}\n\
                 base={:?}\ngrow={:?}\nmin={:?}\nmax={:?}\nfree={}",
                a[i],
                r[i],
                c.base,
                c.grow,
                c.min,
                c.max,
                c.free
            );
        }
    }
}

#[test]
fn analytic_shrink_matches_css_reference() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for case_no in 0..5000 {
        let c = random_case(&mut rng);
        let scaled: Vec<f32> = c.shrink.iter().zip(&c.base).map(|(&s, &b)| s * b).collect();
        let deficit = c.deficit;
        let a = solve_shrink(&c.base, &scaled, &c.min, &c.max, deficit);
        let r = solve_shrink_reference(&c.base, &scaled, &c.min, &c.max, deficit);
        for i in 0..c.base.len() {
            let tol = 1e-3 * a[i].abs().max(r[i].abs()).max(1.0);
            assert!(
                (a[i] - r[i]).abs() <= tol,
                "shrink mismatch (case {case_no}, item {i}): analytic={} ref={}\n\
                 base={:?}\nscaled={:?}\nmin={:?}\nmax={:?}\ndeficit={}",
                a[i],
                r[i],
                c.base,
                scaled,
                c.min,
                c.max,
                deficit
            );
        }
    }
}

#[test]
fn grow_respects_clamps() {
    let base = [10.0, 10.0, 10.0];
    let grow = [1.0, 1.0, 1.0];
    let min = [0.0, 0.0, 0.0];
    let max = [15.0, 100.0, 100.0];
    let out = solve_grow(&base, &grow, &min, &max, 30.0);
    // Child 0 saturates at 15 (it absorbs 5 of the 30), the other two share
    // the remaining 25 => 10 + 12.5 = 22.5 each.
    assert!((out[0] - 15.0).abs() < 1e-4, "{out:?}");
    assert!((out[1] - 22.5).abs() < 1e-4, "{out:?}");
    assert!((out[2] - 22.5).abs() < 1e-4, "{out:?}");
}

#[test]
fn shrink_respects_minimums() {
    let base = [100.0, 100.0];
    let scaled = [1.0, 1.0];
    let min = [80.0, 0.0];
    let max = [200.0, 200.0];
    let out = solve_shrink(&base, &scaled, &min, &max, 100.0);
    // Child 0 hits its minimum of 80 (absorbing 20), child 1 absorbs 80.
    assert!((out[0] - 80.0).abs() < 1e-4, "{out:?}");
    assert!((out[1] - 20.0).abs() < 1e-4, "{out:?}");
}
