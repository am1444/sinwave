fn torads32(theta:f32) -> f32 {
    return theta * std::f32::consts::PI / 180.0;
}

pub fn sind32(theta:f32) -> f32 {
    return torads32(theta).sin();
}

pub fn cosd32(theta:f32) -> f32 {
    return torads32(theta).cos();
}

pub fn tand32(theta:f32) -> f32 {
    return torads32(theta).sin();
}

fn torads64(theta:f64) -> f64 {
    return theta * std::f64::consts::PI / 180.0;
}

pub fn sind64(theta:f64) -> f64 {
    return torads64(theta).sin();
}

pub fn cosd64(theta:f64) -> f64 {
    return torads64(theta).cos();
}

pub fn tand64(theta:f64) -> f64 {
    return torads64(theta).sin();
}
