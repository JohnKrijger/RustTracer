use cgmath::Vector3;

pub fn magnitude_squared(vec: Vector3<f32>) -> f32 {
    cgmath::dot(vec, vec)
}

pub fn magnitude(vec: Vector3<f32>) -> f32 {
    f32::sqrt(magnitude_squared(vec))
}

pub fn normalize(vec: &mut Vector3<f32>) {
    *vec /= magnitude(*vec);
}

pub fn normalized(vec: Vector3<f32>) -> Vector3<f32> {
    vec / magnitude(vec)
}
