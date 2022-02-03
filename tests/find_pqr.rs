use gcd::euclid_u128;

// From this paper: https://www.preprints.org/manuscript/201710.0192/v1
// Choose odd q, even r, and prefer large values with gcd(p, r) = 1
// and pr = q(q-1).
// Identity = -(P/Q)
#[test]
fn find_pqr() {
    // Chosen because it is even and not 2 (which the other combo is using)
    const R: u128 = 8u128;

    fn find_p(q: u128) -> Option<u128> {
        // q is odd
        if q % 2 == 0 {
            return None;
        }

        // p is exact
        if (q * (q - 1)) % R != 0 {
            return None;
        }

        let p = (q * (q - 1)) / R;

        // Identity exists and is round number
        if p % q != 0 {
            return None;
        }

        // Identity is greater than max >> 1;
        if q < 3 {
            return None;
        }

        if euclid_u128(p, R) != 1 {
            return None;
        }

        Some(p)
    }

    let mut q = u64::MAX;
    loop {
        if let Some(p) = find_p(q as u128) {
            println!("{:?}", (p, q));
            break;
        }
        q -= 2;
    }
}
