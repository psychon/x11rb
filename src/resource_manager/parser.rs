use super::{Binding, Component, Entry};

type Resource = Vec<(Binding, Component)>;

#[derive(Debug)]
enum Error {
    Error,
}

fn parse_resource(data: &[u8]) -> Result<(Resource, &[u8]), Error> {
    let _ = data;
    todo!()
}

fn parse_value(data: &[u8]) -> Result<(Entry, &[u8]), Error> {
    let _ = data;
    todo!()
}

fn parse_entry(data: &[u8]) -> Result<(Entry, &[u8]), Error> {
    let (resource, data) = parse_resource(data)?;
    // Now... something about expecting a ':'  and then skipping ' '  and '\t'  until the value
    // begins.
    // Afterwards:
    let value = parse_value(data);
    let _ = (resource, value);
    todo!()
}

#[cfg(test)]
mod test {
    use super::{Binding, Component, parse_entry, parse_resource};

    // The tests in here are based on [1], which is: Copyright © 2016 Ingo Bürk
    // [1]: https://github.com/Airblader/xcb-util-xrm/blob/master/tests/tests_parser.c

    #[test]
    fn test_parse_resource_success() {
        let tests = [
            (b"First.second", vec![
                (Binding::Tight, Component::Normal("First".to_string())),
                (Binding::Tight, Component::Normal("second".to_string())),
            ]),
        ];
        for (data, expected) in tests.iter() {
            match parse_resource(*data) {
                Ok((result, remaining)) => {
                    assert_eq!(remaining, b"");
                    assert_eq!(result, *expected);
                }
                Err(err) => panic!("Failed to parse '{:?}': {:?}", data, err)
            }
        }
    }

    #[test]
    fn test_parse_resource_error() {
        let tests: [&[u8]; 5] = [
            b"First.second: on",
            b"First*second",
            b"First.?.second",
            b"*second",
            b"?.second",
        ];
        for data in tests.iter() {
            match parse_resource(*data) {
                Ok(v) => panic!("Unexpected success parsing '{:?}': {:?}", data, v),
                Err(_) => {},
            }
        }
    }

    #[test]
    fn test_parse_entry_success() {
        let tests: [(&[u8], _, &[u8]); 31] = [
            // Basics
            (
                b"First: 1",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"1",
            ),
            (
                b"First.second: 1",
                vec![
                    (Binding::Tight, Component::Normal("First".to_string())),
                    (Binding::Tight, Component::Normal("second".to_string())),
                ],
                b"1",
            ),
            (
                b"First..second: 1",
                vec![
                    (Binding::Tight, Component::Normal("First".to_string())),
                    (Binding::Tight, Component::Normal("second".to_string())),
                ],
                b"1",
            ),
            // Wildcards
            (
                b"?.second: 1",
                vec![
                    (Binding::Tight, Component::Wildcard),
                    (Binding::Tight, Component::Normal("second".to_string())),
                ],
                b"1",
            ),
            (
                b"First.?.third: 1",
                vec![
                    (Binding::Tight, Component::Normal("First".to_string())),
                    (Binding::Tight, Component::Wildcard),
                    (Binding::Tight, Component::Normal("third".to_string())),
                ],
                b"1",
            ),
            // Loose bindings
            (
                b"*second: 1",
                vec![(Binding::Loose, Component::Normal("second".to_string()))],
                b"1",
            ),
            (
                b"First*third: 1",
                vec![
                    (Binding::Tight, Component::Normal("First".to_string())),
                    (Binding::Loose, Component::Normal("third".to_string())),
                ],
                b"1",
            ),
            (
                b"First**third: 1",
                vec![
                    (Binding::Tight, Component::Normal("First".to_string())),
                    (Binding::Loose, Component::Normal("third".to_string())),
                ],
                b"1",
            ),
            // Combinations
            (
                b"First*?.fourth: 1",
                vec![
                    (Binding::Tight, Component::Normal("First".to_string())),
                    (Binding::Loose, Component::Wildcard),
                    (Binding::Tight, Component::Normal("fourth".to_string())),
                ],
                b"1",
            ),
            // Values
            (
                b"First: 1337",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"1337",
            ),
            (
                b"First: -1337",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"-1337"
            ),
            (
                b"First: 13.37",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"13.37",
            ),
            (
                b"First: value",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"value",
            ),
            (
                b"First: #abcdef",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"#abcdef",
            ),
            (
                b"First: { key: 'value' }",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"{ key: 'value' }",
            ),
            (
                b"First: x?y",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x?y",
            ),
            (
                b"First: x*y",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x*y",
            ),
            // Whitespace
            (
                b"First:    x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x",
            ),
            (
                b"First: x   ",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x   ",
            ),
            (
                b"First:    x   ",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x   ",
            ),
            (
                b"First:x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x",
            ),
            (
                b"First: \t x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x",
            ),
            (
                b"First: \t x \t",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x \t",
            ),
            // Special characters
            (
                b"First: \\ x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b" x",
            ),
            (
                b"First: x\\ x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x x",
            ),
            (
                b"First: \\\tx",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"\tx",
            ),
            (
                b"First: \\011x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"\tx",
            ),
            (
                b"First: x\\\\x",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x\\x",
            ),
            (
                b"First: x\\nx",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"x\nx",
            ),
            (
                b"First: \\080",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"\\080",
            ),
            (
                b"First: \\00a",
                vec![(Binding::Tight, Component::Normal("First".to_string()))],
                b"\\00a",
            ),
        ];
        for (data, resource, value) in tests.iter() {
            run_entry_test(data, resource, value);
        }
    }

    #[test]
    fn test_parse_entry_error() {
        let tests: [&[u8]; 7] = [
            b": 1",
            b"?: 1",
            b"First",
            b"First second",
            b"First.?: 1",
            b"F\xc3\xb6rst: 1",
            b"F~rst: 1",
        ];
        for data in tests.iter() {
            match parse_entry(*data) {
                Ok(v) => panic!("Unexpected success parsing '{:?}': {:?}", data, v),
                Err(_) => {},
            }
        }
    }

    #[test]
    fn test_parse_large_value() {
        let value = vec![b'x'; 1025];
        let mut data = b"First: ".to_vec();
        data.extend(&value);
        let resource = (Binding::Tight, Component::Normal("First".to_string()));
        run_entry_test(&data, &[resource], &value);
    }

    #[test]
    fn test_parse_large_resource() {
        let x = vec![b'x'; 1025];
        let y = vec![b'y'; 1025];
        let mut data = x.clone();
        data.push(b'.');
        data.extend(&y);
        data.extend(b": 1");
        let resource = [
            (Binding::Tight, Component::Normal(String::from_utf8(x).unwrap())),
            (Binding::Tight, Component::Normal(String::from_utf8(y).unwrap())),
        ];
        run_entry_test(&data, &resource, b"1");
    }

    fn run_entry_test(data: &[u8], resource: &[(Binding, Component)], value: &[u8]) {
        match parse_entry(data) {
            Ok((result, remaining)) => {
                assert_eq!(remaining, b"");
                assert_eq!(result.components, resource);
                assert_eq!(result.value, value);
            }
            Err(err) => panic!("Failed to parse '{:?}': {:?}", data, err)
        }
    }
}
