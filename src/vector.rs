pub fn from_mag(magnitude: f32, angle: f32) -> (f32, f32) {
    let x = magnitude * angle.cos();
    let y = magnitude * angle.sin();
    (x, y)
}

pub fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt()
}

pub fn magnitude_of(vector: (f32, f32)) -> f32 {
    (vector.0.powi(2) + vector.1.powi(2)).sqrt()
}

pub fn dot(a: (f32, f32), b: (f32, f32)) -> f32 {
    (a.0 * b.0) + (a.1 * b.1)
}

pub fn angle_between(a: (f32, f32), b: (f32, f32)) -> f32 {
    let norm_a = normalized(a);
    let norm_b = normalized(b);

    dot(norm_a, norm_b).clamp(-1.0, 1.0).acos()
}

pub fn angle_of(vector: (f32, f32)) -> f32 {
    vector.1.atan2(vector.0)
}

pub fn normalized(vector: (f32, f32)) -> (f32, f32) {
    let mag = magnitude_of(vector);
    if mag == 0.0 {
        return (0.0, 0.0);
    }
    (vector.0 / mag, vector.1 / mag)
}

pub fn rotated(vector: (f32, f32), angle: f32) -> (f32, f32) {
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    let x = vector.0 * cos_theta - vector.1 * sin_theta;
    let y = vector.0 * sin_theta + vector.1 * cos_theta;
    (x, y)
}