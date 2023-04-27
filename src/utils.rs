use std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_f32() -> f32 {
    unsafe {
        static mut STATE: u64 = 0x123456789abcdef0;
        STATE = STATE
            .wrapping_mul(2862933555777941757)
            .wrapping_add(3037000493);
        if STATE != 0 {
            STATE -= 1
        }
        ((STATE) as f64 / (u64::MAX as f64)) as f32
    }
}

pub fn random_f32_with_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}
