
#[must_use]
pub fn close_enough(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

const ALMOST_EQUAL_C: u32 = 0x8000_0000_u32;
const ALMOST_EQUAL_CI: i32 = ALMOST_EQUAL_C as i32;

// Compares two f32 values for approximate equality
// Use ULP (Units in the Last Place) comparison.
#[inline]
#[must_use]
pub fn almost_equal_as_int(a: f32, b: f32, ulps: i32) -> bool {
    debug_assert!(a.is_finite());
    debug_assert!(b.is_finite());
    debug_assert!(ulps >= 0);
    
    if a.signum() != b.signum() {
        return a == b;
    }
    
    let mut a_i: i32 = a.to_bits() as i32;
    let mut b_i: i32 = b.to_bits() as i32;
    
    // Make bInt lexicographically ordered as a twos-complement int
    if a_i < 0i32 {
        a_i = ALMOST_EQUAL_CI - a_i;
    }
    if b_i < 0i32 {
        b_i = ALMOST_EQUAL_CI - b_i;
    }
    
    (a_i - b_i).abs() <= ulps
}


pub const FLOAT_INT_EPS: i32 = 10;
#[inline]
#[must_use]
pub fn float_equal(a: f32, b: f32) -> bool {
    almost_equal_as_int(a, b, FLOAT_INT_EPS)
}

#[must_use]
pub fn float_perturbed_as_int(f: f32, c: i32) -> f32 {
    // Special case: f == 0.0 and c == -1 should return -0.0 (valid bit pattern)
    if f == 0.0 && c == -1 {
        return 0.0;
    }
    if f == -0.0 && c == 1 {
        return 0.0;
    }
    let mut f_i: i32 = f.to_bits() as i32;
    f_i += c;
    f32::from_bits(f_i as u32)
}

#[must_use]
pub fn float_prev(f: f32) -> f32 {
    float_perturbed_as_int(f, -FLOAT_INT_EPS)
}


#[must_use]
pub fn float_next(f: f32) -> f32 {
    float_perturbed_as_int(f, FLOAT_INT_EPS)
}

