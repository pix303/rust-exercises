pub fn sum(a: u8, b: u8) -> u16 {
    let r = a + b;
    r as u16
}

pub fn sub(a: u8, b: u8) -> i16 {
    let aa: i16 = a as i16;
    let bb: i16 = b as i16;
    aa - bb
}

pub fn multiply(a: u8, b: u8) -> u64 {
    (a * b) as u64
}

pub fn divide(a: u8, b: u8) -> f32 {
    let aa: f32 = a as f32;
    let bb: f32 = b as f32;
    aa / bb
}
