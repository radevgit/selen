
#[must_use]
pub fn close_enough(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}