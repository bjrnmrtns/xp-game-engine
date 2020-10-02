pub fn get_lowest_root(a: f32, b: f32, c: f32, max_r: f32) -> Option<f32> {
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {
        return None;
    }
    let sqrt_det = det.sqrt();
    let r1 = (-b - sqrt_det) / (2.0 * a);
    let r2 = (-b + sqrt_det) / (2.0 * a);
    let (r1, r2) = if r1 > r2 { (r2, r1) } else { (r1, r2) };
    if (r1 > 0.0 && r1 <= max_r) {
        return Some(r1);
    }
    if (r2 > 0.0 && r2 <= max_r) {
        return Some(r2);
    }
    None
}
