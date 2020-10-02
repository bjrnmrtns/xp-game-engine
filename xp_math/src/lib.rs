pub fn get_roots(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {
        return None;
    }
    let sqrt_det = det.sqrt();
    let r0 = (-b - sqrt_det) / (2.0 * a);
    let r1 = (-b + sqrt_det) / (2.0 * a);
    Some((r0, r1))
}
