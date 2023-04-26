#[cfg(test)]
mod tests {

    use fixed::types::I32F32;
    use rphx::linear::FP64;

    #[test]
    fn it_works() {
        let a: FP64 = FP64::new(-1);
        let b = FP64::new(31415926);
        let c = FP64::new(10000000);
        let d = a * b / c;
        let tar = FP64::new_i32_f32(I32F32::from_num(-3.1415926));
        assert_eq!(d, tar);
    }
}
