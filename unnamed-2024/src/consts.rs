pub mod drive {
    pub mod limit {
        use crate::prelude::*;

        pub const MAX_SPEED: Velocity = Velocity::from_one::<Meter, Second>(3.0);
        pub const MAX_ROTATION: AngVelocity =
            AngVelocity::from_one::<Radian, Second>(f64::consts::TAU);
    }

    pub mod slew {
        use crate::prelude::*;

        pub const DIRECTION: AngVelocity = AngVelocity::from_one::<Radian, Second>(1.2);
        pub const MAGNITUDE: f64 = 1.8; // percent per second
        pub const ROTATION: f64 = 2.0; // percent per second
    }

    pub mod chassis {
        use crate::prelude::*;

        pub const TRACK_WIDTH: Length = Length::from::<Inch>(21.5);
        pub const WHEEL_BASE: Length = Length::from::<Inch>(21.5);
    }

    pub mod offsets {
        use crate::prelude::*;

        pub const FRONT_LEFT: f64 = -f64::consts::PI / 2.0;
        pub const FRONT_RIGHT: f64 = 0.0;
        pub const BACK_LEFT: f64 = f64::consts::PI;
        pub const BACK_RIGHT: f64 = f64::consts::PI / 2.0;
    }

    pub mod ids {
        pub mod driving {
            pub const FRONT_LEFT: u8 = 10;
            pub const FRONT_RIGHT: u8 = 20;
            pub const BACK_LEFT: u8 = 30;
            pub const BACK_RIGHT: u8 = 40;
        }

        pub mod turning {
            pub const FRONT_LEFT: u8 = 15;
            pub const FRONT_RIGHT: u8 = 25;
            pub const BACK_LEFT: u8 = 35;
            pub const BACK_RIGHT: u8 = 45;
        }
    }

    pub mod gyro {
        pub const REVERSED: bool = false;
    }
}

pub mod module {
    pub mod driving {
        pub mod motor {
            use super::super::teeth::*;
            use crate::prelude::*;

            /// in RPS
            pub const FREE_SPEED: f64 = 5676.0 / 60.0;
            pub const REDUCTION: f64 =
                ((WHEEL_BEVEL * SPUR) / (DRIVING_PINION * BEVEL_PINION)) as f64;
        }

        pub mod encoder {
            use crate::prelude::*;
        }
    }

    pub mod teeth {
        pub const DRIVING_PINION: u16 = 14;
        pub const WHEEL_BEVEL: u16 = 45;
        pub const SPUR: u16 = 22;
        pub const BEVEL_PINION: u16 = 15;
    }

    pub mod wheel {
        use super::driving::motor;
        use crate::prelude::*;

        pub const DIAMETER: Length = Length::from::<Inch>(3.0);
        pub const CIRCUMFERENCE: Length = Length::from::<Inch>(3.0 * f64::consts::PI);

        /// rps
        pub const FREE_SPEED: f64 =
            (motor::FREE_SPEED * CIRCUMFERENCE.to::<Meter>()) / motor::REDUCTION;
    }

    pub mod pid {
        pub mod driving {
            pub const P: f64 = 0.15;
            pub const I: f64 = 0.0001;
            pub const D: f64 = 0.01;
        }

        pub mod turning {
            pub const P: f64 = 4.0;
            pub const I: f64 = 0.0001;
            pub const D: f64 = 0.02;
        }
    }
}
