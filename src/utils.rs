
const TWO_COMPLEMENT: u32 = 0x8000_0000_u32;
const TWO_COMPLEMENT_CI: i32 = TWO_COMPLEMENT as i32;

// Compares two f32 values for approximate equality
// Use ULP (Units in the Last Place) comparison.
#[inline]
#[must_use]
pub fn almost_equal_as_int(a: f32, b: f32, ulps: u32) -> bool {
    debug_assert!(a.is_finite());
    debug_assert!(b.is_finite());

    if a.signum() != b.signum() {
        return a == b;
    }

    let mut a_i: i32 = a.to_bits() as i32;
    let mut b_i: i32 = b.to_bits() as i32;

    // Make bInt lexicographically ordered as a twos-complement int
    if a_i < 0i32 {
        a_i = TWO_COMPLEMENT_CI - a_i;
    }
    if b_i < 0i32 {
        b_i = TWO_COMPLEMENT_CI - b_i;
    }

    (a_i - b_i).abs() <= ulps as i32
}

pub const FLOAT_INT_EPS: u32 = 10;
#[inline]
#[must_use]
pub fn float_equal(a: f32, b: f32) -> bool {
    almost_equal_as_int(a, b, FLOAT_INT_EPS)
}

#[must_use]
pub fn float_perturbed_as_int(f: f32, c: i32) -> f32 {
    debug_assert!(f.is_finite());

    if c == 0 {
        return f;
    }

    // Special cases for zero crossings in ULP ordering:
    // +0.0 with -1 perturbation should give -0.0
    // -0.0 with +1 perturbation should give +0.0
    if f == 0.0 && c == -1 {
        return -0.0;
    }
    if f == -0.0 && c == 1 {
        return 0.0;
    }

    // Convert to the same lexicographically ordered space as almost_equal_as_int
    let f_bits = f.to_bits();
    let f_i = f_bits as i32;

    // Convert to lexicographically ordered space (same as almost_equal_as_int)
    let lex_value = if f_i < 0 {
        TWO_COMPLEMENT_CI - f_i
    } else {
        f_i
    };

    // Apply perturbation in lexicographic space
    let result_lex = lex_value + c;

    // Convert back from lexicographically ordered space to IEEE float bits
    let result_bits = if result_lex < 0 {
        // Result is negative in lex space, convert back to IEEE negative representation
        (TWO_COMPLEMENT_CI - result_lex) as u32
    } else {
        // Result is positive in lex space, it's already in IEEE positive representation
        result_lex as u32
    };

    f32::from_bits(result_bits)
}

#[must_use]
pub fn float_prev(f: f32) -> f32 {
    let eps = -(FLOAT_INT_EPS as i32) - 1;
    float_perturbed_as_int(f, eps)
}

#[must_use]
pub fn float_next(f: f32) -> f32 {
    let eps = FLOAT_INT_EPS as i32 + 1;
    float_perturbed_as_int(f, eps)
}
