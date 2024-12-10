use std::{convert, fmt, path::Display, str::FromStr};

#[derive(Clone, Debug)]
pub struct Input {
    pub data: Vec<(Option<usize>, usize)>,
}

impl Input {
    fn process(&self) -> Vec<Option<usize>> {
        let mut data = vec![];

        for (num, len) in &self.data {
            data.extend(vec![*num; *len]);
        }

        data
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut empty = false;
        let mut data = vec![];
        let mut i = 0;

        for ch in s.chars() {
            let len = ch as u8 - b'0';

            if empty {
                data.push((None, len as usize));
            } else {
                data.push((Some(i), len as usize));
                i += 1;
            }

            empty = !empty;
        }

        Ok(Input { data })
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (d, len) in &self.data {
            if *len == 0 {
                continue;
            }

            let s = d
                .as_ref()
                .map(usize::to_string)
                .unwrap_or(String::from("."));

            let rep = s.repeat(len - 1);

            // print s in red and rep in white
            write!(f, "\x1b[31m{}\x1b[0m{}", s, rep)?;
        }

        Ok(())
    }
}

pub fn level1(input: Input) -> usize {
    let mut input = input.process();
    let mut j = 0;
    let mut i = input.len() - 1;

    loop {
        while input[j].is_some() {
            j += 1;
        }

        while input[i].is_none() {
            i -= 1;
        }

        if i < j {
            break;
        }

        input.swap(i, j);
    }

    let mut sum = 0;
    for (i, item) in input.iter().enumerate() {
        let Some(item) = item else {
            break;
        };

        sum += i * item;
    }

    sum
}

pub fn level2(mut input: Input) -> usize {
    let mut data = input.data.clone();

    let mut j = data.len() - 1;
    let mut i = 0;

    loop {
        fn jump_i(i: &mut usize, data: &[(Option<usize>, usize)]) {
            if *i != 0 {
                *i += 1;
            }

            if *i >= data.len() {
                return;
            }

            while data[*i].0.is_some() {
                *i += 1;

                if *i >= data.len() {
                    break;
                }
            }
        }

        fn jump_j(j: &mut usize, data: &[(Option<usize>, usize)]) {
            *j -= 1;

            while data[*j].0.is_none() {
                *j -= 1;

                if *j == 0 {
                    break;
                }
            }
        }

        fn jump(i: &mut usize, j: &mut usize, data: &Vec<(Option<usize>, usize)>) {
            *i = 0;

            while *i < *j && (data[*i].1 < data[*j].1 || data[*i].0.is_some()) {
                jump_i(i, data);
            }

            if *i >= data.len() || *i >= *j || data[*i].1 < data[*j].1 || data[*j].0.is_none() {
                jump_j(j, data);

                if *j == 0 {
                    return;
                }

                jump(i, j, data);
            }
        }

        fn swap_or_insert(i: &mut usize, j: &mut usize, data: &mut Vec<(Option<usize>, usize)>) {
            if data[*i].1 == data[*j].1 {
                data.swap(*i, *j);

                if data[*j - 1].0.is_none() {
                    data[*j].1 += data[*j - 1].1;
                    data.remove(*j - 1);
                    *j -= 1;
                }

                if *j + 1 < data.len() && data[*j + 1].0.is_none() {
                    data[*j].1 += data[*j + 1].1;
                    data.remove(*j + 1);
                }
            } else {
                data.insert(*i, data[*j]);
                *i += 1;
                *j += 1;

                data[*i].1 -= data[*j].1;
                data[*j].0 = None;

                if *j + 1 < data.len() && data[*j + 1].0.is_none() {
                    data[*j].1 += data[*j + 1].1;
                    data.remove(*j + 1);
                }

                *j -= 1;
            }
        }

        jump(&mut i, &mut j, &data);

        if j == 0 {
            break;
        }

        swap_or_insert(&mut i, &mut j, &mut data);
    }

    let data = Input { data }.process();
    let mut sum = 0;

    for (i, item) in data.iter().enumerate() {
        let Some(item) = item else {
            continue;
        };

        sum += i * item;
    }

    sum
}
