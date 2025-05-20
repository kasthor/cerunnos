use rand::Rng;

pub fn mutate_usize(current: usize, min: usize, max: usize, strength: f64, rng: &mut impl Rng) -> usize {
    let change = (rng.random_range(-1.0..1.0) * strength * (max - min) as f64).round() as isize;
    let new_val = current as isize + change;

    new_val.clamp(min as isize, max as isize) as usize
}

pub fn mutate_f64(current: f64, min: f64, max: f64, strength: f64, rng: &mut impl Rng) -> f64 {
    let change = rng.random_range(-1.0..1.0) * strength * (max - min);
    (current + change).clamp(min, max)
}
