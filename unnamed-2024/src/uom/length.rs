pub trait LengthUnit {
    const MM_PER_UNIT: f64;
}

pub struct Inch;
pub struct Meter;
pub struct Foot;

impl LengthUnit for Inch {
    const MM_PER_UNIT: f64 = 25.4;
}

impl LengthUnit for Meter {
    const MM_PER_UNIT: f64 = 1_000.0;
}

impl LengthUnit for Foot {
    const MM_PER_UNIT: f64 = 304.8;
}
