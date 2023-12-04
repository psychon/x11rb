#[cfg(feature = "xinput")]
#[test]
fn test_fp1616() {
    use x11rb_protocol::protocol::xinput::fp1616_as_f32;

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
    }
}

#[cfg(feature = "xinput")]
#[test]
fn test_fp3232() {
    use x11rb_protocol::protocol::xinput::Fp3232;

    let tests = [
        (0i64, 0.),
        (1 << 32, 1.),
        (-1 << 32, -1.),
        (1 << 31, 0.5),
        (-1 << 31, -0.5),
        (3 << 31, 1.5),
        (-3 << 31, -1.5),
        // The following actually tests a rounding error since the result should be slightly less
        // than 2^31.
        (i64::MAX, 2f64.powi(31)),
        (i64::MIN, -2f64.powi(31)),
        // The largest number below 1
        ((1 << 32) - 1, 1f64 - (1f64 / ((1u64 << 32) as f64))),
        (-(1 << 32) + 1, -1f64 + (1f64 / ((1u64 << 32) as f64))),
    ];

    for (input, output) in tests {
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
}
