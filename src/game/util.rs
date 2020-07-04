use super::sfml::system::Vector2f;


pub fn vector_length(vec: &Vector2f) -> f32 {
    (vec.x * vec.x + vec.y * vec.y).sqrt()
}

pub fn vector_normalize(vec: &Vector2f) -> Vector2f {
    let len = vector_length(vec);
    Vector2f::new(vec.x / len, vec.y / len)
}

pub fn vector_normalize_with_len(vec: &Vector2f, len: f32) -> Vector2f {
    Vector2f::new(vec.x / len, vec.y / len)
}
