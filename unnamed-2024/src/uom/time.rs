pub trait TimeUnit {
    const SEC_PER_UNIT: f64;
}

pub struct Millisecond;
pub struct Second;
pub struct Minute;
pub struct Hour;

impl TimeUnit for Millisecond {
    const SEC_PER_UNIT: f64 = 0.001;
}

impl TimeUnit for Second {
    const SEC_PER_UNIT: f64 = 1.0;
}

impl TimeUnit for Minute {
    const SEC_PER_UNIT: f64 = 60.0;
}

impl TimeUnit for Hour {
    const SEC_PER_UNIT: f64 = 3_600.0;
}
