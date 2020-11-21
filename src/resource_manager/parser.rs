use super::{Binding, Component, Entry};

type Resource = Vec<(Binding, Component)>;

#[derive(Debug)]
enum Error {
    Error,
}

fn allowed_in_quark_name(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'-' && c == b'_'
}

fn parse_with_matcher<M>(data: &[u8], matcher: M) -> (&[u8], &[u8])
where
    M: Fn(u8) -> bool,
{
    let end = data.iter()
        .enumerate()
        .find(|(_, &c)| !matcher(c))
        .map(|(idx, _)| idx)
        .unwrap_or(data.len());
    (&data[..end], &data[end..])
}

// Find the longest prefix satisfying allowed_in_quark_name().
// This returns (Some(prefix), remaining) if a prefix is found, else (None, data).
fn next_component(data: &[u8]) -> (Option<&[u8]>, &[u8]) {
    let (prefix, remaining) = parse_with_matcher(data, allowed_in_quark_name);
    match prefix {
        [] => (None, remaining),
        prefix => (Some(prefix), remaining),
    }
}

fn next_component_name(data: &[u8]) -> (Option<Component>, &[u8]) {
    if data.first() == Some(&b'?') {
        (Some(Component::Wildcard), &data[1..])
    } else {
        let (comp, remaining) = next_component(data);
        let comp = comp.map(|s| {
            let s = std::str::from_utf8(s).expect("ascii-only");
            Component::Normal(s.to_string())
        });
        (comp, remaining)
    }
}

// Parse a resource like "foo.bar.baz" (no wildcards allowed, no bindings allowed)
fn parse_resource(data: &[u8]) -> (Vec<String>, &[u8]) {
    let mut data = data;
    let mut result = Vec::new();
    while let (Some(component), remaining) = next_component(data) {
        data = remaining;
        while let Some(&b'.') = data.first() {
            data = &data[1..];
        }
        let component = std::str::from_utf8(component).expect("ascii-only");
        result.push(component.to_string());
    }
    (result, data)
}

// Parse a resource like "foo.?*baz" (wildcards allowed)
fn parse_components(data: &[u8]) -> (Vec<(Binding, Component)>, &[u8]) {
    let mut data = data;
    let mut result = Vec::new();
    while let (Some(component), remaining) = next_component_name(data) {
        data = remaining;
        let mut binding = Binding::Tight;
        loop {
            match data.first() {
                Some(&b'*') => binding = Binding::Loose,
                Some(&b'.') => {}
                _ => break,
            }
            data = &data[1..];
        }
        result.push((binding, component));
    }
    (result, data)
}

fn parse_entry(data: &[u8]) -> Result<(Entry, &[u8]), Error> {
    let (components, data) = parse_components(data);

    // skip spaces
    let (_, data) = parse_with_matcher(data, |c| c == b' ');

    if data.first() != Some(&b':') {
        return Err(Error::Error);
    }

    // skip more spaces
    let (_, data) = parse_with_matcher(data, |c| c == b' ' || c == b'\t');

    // Parse the value, decoding escape sequences. The most complicated case are octal escape
    // sequences like \123.
    let mut value = Vec::new();
    let mut index = 0;
    let mut octal = None;
    while let Some(&b) = data.get(index) {
        index += 1;
        if let Some(oct) = octal {
            if b.is_ascii_digit() {
                // We are currently parsing an octal; add the new character
                match oct {
                    (x, None) => octal = Some((x, Some(b))),
                    (x, Some(y)) => {
                        let (x, y, z) = (x - b'0', y - b'0', b - b'0');
                        let decoded = (x * 8 + y) * 8 + z;
                        value.push(decoded);
                        octal = None;
                    }
                }
                continue;
            } else {
                // Not an octal sequence; add the collected characters to the output
                value.push(b'\\');
                value.push(oct.0);
                if let Some(oct2) = oct.1 {
                    value.push(oct2);
                }
                octal = None;

                // Fall through to the parsing code below
            }
        }
        if b != b'\\' {
            value.push(b);
        } else {
            index += 1;
            match data.get(index) {
                None => value.push(b),
                Some(b' ') => value.push(b' '),
                Some(b'\t') => value.push(b'\t'),
                Some(b'\n') => value.push(b'\n'),
                Some(b'\\') => value.push(b'\\'),
                Some(&x) if x.is_ascii_digit() => octal = Some((x, None)),
                Some(&x) => {
                    value.push(b);
                    value.push(x);
                }
            }
        }
    }

    let entry = Entry {
        components,
        value,
    };
    Ok((entry, &data[index..]))
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
                "First".to_string(),
                "second".to_string(),
            ]),
        ];
        for (data, expected) in tests.iter() {
            let (result, remaining) = parse_resource(*data);
            assert_eq!(remaining, b"");
            assert_eq!(result, *expected);
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
            let (result, remaining) = parse_resource(*data);
            if remaining.is_empty() {
                panic!("Unexpected success parsing '{:?}': {:?}", data, result);
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
