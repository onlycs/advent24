problem_parser!(ty_parser!(String) => String);

pub fn level1(input: String) -> i32 {
    let mut input = input.as_str();
    let mut sum = 0;

    while let Some(idx) = input.find("mul(") {
        let idx = idx + 4;

        if let Some(closing) = input[idx..].find(')')
            && let closing = idx + closing
            && let Some(comma) = input[idx..closing].find(',')
            && let comma = idx + comma
            && let Ok(lhs) = input[idx..comma].parse::<i32>()
            && let Ok(rhs) = input[comma + 1..closing].parse::<i32>()
        {
            sum += lhs * rhs;
            input = &input[closing + 1..];
        } else {
            input = &input[idx..];
        }
    }

    sum
}

pub fn level2(input: String) -> i32 {
    let mut input = input.as_str();
    let mut sum = 0;
    let mut state = true;

    'outer: while !input.is_empty() {
        if state && input.starts_with("mul(") {
            input = &input[4..];

            let mut lhs = 0;
            let mut rhs = 0;

            while let b = input.as_bytes()[0]
                && b != b','
            {
                input = &input[1..];
                if !(b as char).is_numeric() {
                    continue 'outer;
                }

                lhs *= 10;
                lhs += (b - b'0') as i32;
            }
            input = &input[1..];

            while let b = input.as_bytes()[0]
                && b != b')'
            {
                input = &input[1..];
                if !(b as char).is_numeric() {
                    continue 'outer;
                }

                rhs *= 10;
                rhs += (b - b'0') as i32;
            }

            input = &input[1..];

            sum += lhs * rhs;
        } else if input.starts_with("do()") {
            input = &input[4..];
            state = true;
        } else if input.starts_with("don't()") {
            input = &input[7..];
            state = false;
        } else {
            input = &input[1..];
        }
    }

    sum
}
