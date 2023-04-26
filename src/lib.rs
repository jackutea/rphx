/// Define: right   is FPVector3(1, 0, 0)
/// Define: up      is FPVector3(0, 1, 0)
/// Define: forward is FPVector3(0, 0, 1)
pub mod linear {

    use fixed::types::I32F32;
    use std::{
        fmt::{Debug, Formatter},
        ops::{Add, Div, Mul, Sub},
    };

    /// FP64
    /// FP64 is a fixed point number with 64 bits of precision, Q31.32
    pub struct FP64(I32F32);
    impl FP64 {
        pub fn new(n: i32) -> FP64 {
            FP64(I32F32::from_num(n))
        }
        pub fn new_i32_f32(n: I32F32) -> FP64 {
            FP64(n)
        }
        pub fn en1() -> FP64 {
            FP64(I32F32::from_num(1)) / FP64(I32F32::from_num(10))
        }
    }
    impl Add for FP64 {
        type Output = FP64;
        fn add(self, rhs: FP64) -> Self::Output {
            FP64(self.0 + rhs.0)
        }
    }
    impl Sub for FP64 {
        type Output = FP64;
        fn sub(self, rhs: FP64) -> Self::Output {
            FP64(self.0 - rhs.0)
        }
    }
    impl Div for FP64 {
        type Output = FP64;
        fn div(self, rhs: FP64) -> Self::Output {
            FP64(self.0 / rhs.0)
        }
    }
    impl Mul for FP64 {
        type Output = FP64;
        fn mul(self, rhs: FP64) -> Self::Output {
            FP64(self.0 * rhs.0)
        }
    }
    // 重载比较运算符
    impl PartialEq for FP64 {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    // 重载 Debug
    impl Debug for FP64 {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.to_num::<f64>())
        }
    }
    // 重载 Clone
    impl Clone for FP64 {
        fn clone(&self) -> Self {
            FP64(self.0)
        }
    }

    pub struct FPVector2 {
        pub x: FP64,
        pub y: FP64,
    }

    pub struct FPVector3 {
        pub x: FP64,
        pub y: FP64,
        pub z: FP64,
    }

    impl FPVector3 {
        pub fn new(x: FP64, y: FP64, z: FP64) -> FPVector3 {
            FPVector3 { x, y, z }
        }
        pub fn right() -> FPVector3 {
            FPVector3::new(FP64::new(1), FP64::new(0), FP64::new(0))
        }
        pub fn up() -> FPVector3 {
            FPVector3::new(FP64::new(0), FP64::new(1), FP64::new(0))
        }
        pub fn forward() -> FPVector3 {
            FPVector3::new(FP64::new(0), FP64::new(0), FP64::new(1))
        }
    }

    pub struct FPVector4 {
        pub x: FP64,
        pub y: FP64,
        pub z: FP64,
        pub w: FP64,
    }

    pub struct FPMatrix4x4 {
        pub m: [[FP64; 4]; 4],
    }

    pub struct FPQuaternion {
        pub x: FP64,
        pub y: FP64,
        pub z: FP64,
        pub w: FP64,
    }
}
