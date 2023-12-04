#[cfg(feature = "xinput")]
#[test]
fn test_fp1616() {
    use x11rb_protocol::protocol::xinput::{f32_as_fp1616, fp1616_as_f32};

    let tests = [
        (0, 0.),
        (1 << 16, 1.),
        (-1 << 16, -1.),
        (1 << 15, 0.5),
        (-1 << 15, -0.5),
        (3 << 15, 1.5),
        (-3 << 15, -1.5),
        (i32::MAX, 2f32.powi(15)),
        (i32::MIN, -2f32.powi(15)),
    ];

    for (input, output) in tests {
        assert_eq!(
            output,
            fp1616_as_f32(input),
            "Expected fp1616_as_f32({input}) to be {output}"
        );
        assert_eq!(
            input,
            f32_as_fp1616(output),
            "Expected f32_as_fp1616({output}) to be {input}"
        );
    }
}

#[cfg(feature = "xinput")]
#[test]
fn test_fp3232() {
    use x11rb_protocol::protocol::xinput::Fp3232;

    fn test_to_f64(input: i64, output: f64) {
        let bytes = input.to_be_bytes();
        let integral = i32::from_be_bytes(bytes[..4].try_into().unwrap());
        let frac = u32::from_be_bytes(bytes[4..].try_into().unwrap());
        let obj = Fp3232 { integral, frac };
        assert_eq!(
            output,
            obj.as_f64(),
            "Expected Fp3232 {{ {input} }} == Fp3232 {{ integral: {integral}, frac: {frac} }}.as_f64() to be {output}"
        );
    }

    fn test_from_f64(input: i64, output: f64) {
        let obj = Fp3232::from_f64(output);
        let bytes = input.to_be_bytes();
        let integral = i32::from_be_bytes(bytes[..4].try_into().unwrap());
        let frac = u32::from_be_bytes(bytes[4..].try_into().unwrap());
        assert_eq!((obj.integral, obj.frac), (integral, frac),
            "Expected Fp3232::from_f64({output}) == Fp3232 {{ integral: {integral}, frac: {frac} }} == {input} as fp3232"
            );
    }

    let tests = [
        (0i64, 0.),
        (1 << 32, 1.),
        (-1 << 32, -1.),
        (1 << 31, 0.5),
        (-1 << 31, -0.5),
        (3 << 31, 1.5),
        (-3 << 31, -1.5),
        (1 << 60, 2f64.powi(28)),
        (-1 << 60, -2f64.powi(28)),
        (
            (1 << 40) - 42,
            2f64.powi(8) - (42f64 / ((1u64 << 32) as f64)),
        ),
        (
            (-1 << 40) + 42,
            -2f64.powi(8) + (42f64 / ((1u64 << 32) as f64)),
        ),
        (1 << 62, 2f64.powi(30)),
        (-1 << 62, -2f64.powi(30)),
        // The largest number below 1
        ((1 << 32) - 1, 1f64 - (1f64 / ((1u64 << 32) as f64))),
        // The smallest number above -1
        (-(1 << 32) + 1, -1f64 + (1f64 / ((1u64 << 32) as f64))),
    ];

    for (input, output) in tests {
        test_to_f64(input, output);
        test_from_f64(input, output);
    }

    // These fail due to... issues
    test_to_f64(i64::MAX, 2f64.powi(31));
    test_to_f64(i64::MIN, -2f64.powi(31));
    test_to_f64(i64::MAX / 2, 2f64.powi(30));
    test_to_f64(i64::MIN / 2, -2f64.powi(30));
}
