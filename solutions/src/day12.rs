use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, Index},
    sync::Arc,
};

use libadvent::{AsInput, NewlineSeperated, Single};

pub struct Input {
    inner: Vec<Vec<char>>,
}

impl AsInput for Input {
    type Input = Self;

    fn from_str(s: &str) -> Self::Input {
        Self {
            inner: NewlineSeperated::<Single<char>>::from_str(s),
        }
    }
}

impl Index<(usize, usize)> for Input {
    type Output = char;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.inner[i][j]
    }
}

impl Index<usize> for Input {
    type Output = Vec<char>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.inner[i]
    }
}

impl Deref for Input {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct FloodFill {
    seen: HashSet<(usize, usize)>,
    searching: Vec<(usize, usize)>,
    input: Arc<Input>,
    ch: char,
}

impl FloodFill {
    fn new(input: Arc<Input>, ch: char) -> Self {
        Self {
            seen: HashSet::new(),
            searching: Vec::new(),
            input,
            ch,
        }
    }

    fn surrounding(&mut self, (y, x): (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let mut v = [None; 4];
        let filterer = |n: &(usize, usize)| {
            let seen = self.seen.contains(n);
            let isch = self.input[*n] == self.ch;

            !seen && isch
        };

        if y > 0 {
            v[0] = Some((y - 1, x)).filter(filterer);
        }

        if x > 0 {
            v[1] = Some((y, x - 1)).filter(filterer);
        }

        if y < self.input.len() - 1 {
            v[2] = Some((y + 1, x)).filter(filterer);
        }

        if x < self.input[0].len() - 1 {
            v[3] = Some((y, x + 1)).filter(filterer);
        }

        v
    }

    fn search(&mut self) {
        let coord = self.searching.pop().unwrap();

        self.surrounding(coord).into_iter().flatten().for_each(|n| {
            self.seen.insert(n);
            self.searching.push(n);
        });
    }

    fn start_at(&mut self, coords: (usize, usize)) {
        self.searching.push(coords);
        self.seen.insert(coords);

        while !self.searching.is_empty() {
            self.search();
        }
    }

    fn perim(&self) -> usize {
        let mut hfences = HashMap::new();
        let mut vfences = HashMap::new();

        for (i, j) in self.seen.iter().copied() {
            hfences.entry((i, j)).and_modify(|n| *n += 1).or_insert(1);
            hfences
                .entry((i + 1, j))
                .and_modify(|n| *n += 1)
                .or_insert(1usize);

            vfences.entry((i, j)).and_modify(|n| *n += 1).or_insert(1);
            vfences
                .entry((i, j + 1))
                .and_modify(|n| *n += 1)
                .or_insert(1usize);
        }

        hfences.values().filter(|n| **n == 1).count()
            + vfences.values().filter(|n| **n == 1).count()
    }

    fn nsides(&self) -> usize {
        // NOTE TO SELF, 0=up, 1=right, 2=down, 3=left
        let mut sides = HashSet::<((usize, usize), usize)>::new();

        for (y, x) in self.seen.iter().copied() {
            let up = (y.max(1) - 1, x);
            let right = (y, x.min(self.input[0].len() - 2) + 1);
            let down = (y.min(self.input.len() - 2) + 1, x);
            let left = (y, x.max(1) - 1);

            if !self.seen.contains(&up) || y == 0 {
                sides.insert(((y, x), 0));
            }

            if !self.seen.contains(&right) || x == self.input[0].len() - 1 {
                sides.insert(((y, x), 1));
            }

            if !self.seen.contains(&down) || y == self.input.len() - 1 {
                sides.insert(((y, x), 2));
            }

            if !self.seen.contains(&left) || x == 0 {
                sides.insert(((y, x), 3));
            }
        }

        sides
            .iter()
            .copied()
            .filter(|((y, x), dir)| match dir {
                0 | 2 => *x == 0 || !sides.contains(&((*y, x - 1), *dir)),
                1 | 3 => *y == 0 || !sides.contains(&((y - 1, *x), *dir)),
                _ => unreachable!(),
            })
            .count()

        // let isbrokenx = |[p0, p1]: &[(usize, usize); 2]| {
        //     let (y1, x1) = *p0;
        //     let (y2, x2) = *p1;

        //     if x1 + 1 != x2 {
        //         return true;
        //     }

        //     if y1 >= self.input.len() - 1
        //         || y2 >= self.input.len() - 1
        //         || x1 >= self.input[0].len() - 1
        //         || x2 >= self.input[0].len() - 1
        //         || y1 == 0
        //         || y2 == 0
        //         || x1 == 0
        //         || x2 == 0
        //     {
        //         return false; // this is on purpose
        //     }

        //     let above0 = (y1 - 1, x1);
        //     let above1 = (y2 - 1, x2);

        //     if !((self.input[above0] == self.ch && self.input[above1] == self.ch)
        //         || (self.input[*p0] == self.ch && self.input[*p1] == self.ch))
        //     {
        //         return true;
        //     }

        //     false
        // };

        // let isbrokeny = |[p0, p1]: &[(usize, usize); 2]| {
        //     let (y1, x1) = *p0;
        //     let (y2, x2) = *p1;

        //     if y1 + 1 != y2 {
        //         return true;
        //     }

        //     if y1 >= self.input.len() - 1
        //         || y2 >= self.input.len() - 1
        //         || x1 >= self.input[0].len() - 1
        //         || x2 >= self.input[0].len() - 1
        //         || y1 == 0
        //         || y2 == 0
        //         || x1 == 0
        //         || x2 == 0
        //     {
        //         return false; // this is on purpose
        //     }

        //     let left0 = (y1, x1 - 1);
        //     let left1 = (y2, x2 - 1);

        //     if !((self.input[left0] == self.ch && self.input[left1] == self.ch)
        //         || (self.input[*p0] == self.ch && self.input[*p1] == self.ch))
        //     {
        //         return true;
        //     }

        //     false
        // };

        // let hgood = hfences.into_iter().filter(|(_, l)| *l == 1).map(|(k, _)| k);
        // let hchnk = hgood.sorted_by_key(|(y, _)| *y).chunk_by(|(y, _)| *y);
        // let hsort = hchnk.into_iter().map(|(_, c)| c.sorted_by_key(|(_, x)| *x));
        // let hfilt = hsort.map(|c| c.map_windows::<_, _, 2>(isbrokenx).filter(mem::copy));
        // let htotal = hfilt.map(|c| c.count() + 1).sum::<usize>();

        // let vgood = vfences.into_iter().filter(|(_, l)| *l == 1).map(|(k, _)| k);
        // let vchnk = vgood.sorted_by_key(|(_, x)| *x).chunk_by(|(_, x)| *x);
        // let vsort = vchnk.into_iter().map(|(_, c)| c.sorted_by_key(|(y, _)| *y));
        // let vfilt = vsort.map(|c| c.map_windows::<_, _, 2>(isbrokeny).filter(mem::copy));
        // let vtotal = vfilt.map(|c| c.count() + 1).sum::<usize>();

        // println!("{htotal} {vtotal}");

        // htotal + vtotal
    }
}

pub type Parser = Input;

pub fn level1(input: Input) -> usize {
    let mut price = 0;
    let mut seen = HashSet::new();
    let input = Arc::new(input);

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if seen.contains(&(i, j)) {
                continue;
            }

            let mut search = FloodFill::new(Arc::clone(&input), input[(i, j)]);
            search.start_at((i, j));

            let perim = search.perim();
            price += perim * search.seen.len(); // perim * aria

            seen.extend(search.seen);
        }
    }

    price
}

pub fn level2(input: Input) -> usize {
    let mut price = 0;
    let mut seen = HashSet::new();
    let input = Arc::new(input);

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if seen.contains(&(i, j)) {
                continue;
            }

            let mut search = FloodFill::new(Arc::clone(&input), input[(i, j)]);
            search.start_at((i, j));

            let nsides = search.nsides();
            price += nsides * search.seen.len(); // perim * aria

            seen.extend(search.seen);
        }
    }

    price
}
