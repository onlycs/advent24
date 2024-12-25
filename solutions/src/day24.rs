use libadvent::{FuncParser, IsInput, Parser, Seperated};

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Operation {
    And,
    Or,
    Xor,
}

impl Operation {
    fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::Xor => a != b,
        }
    }
}

impl IsInput for Operation {
    fn parse(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!(),
        }
    }
}

pub struct Input {
    resolved: HashMap<String, bool>,
    dependencies: HashMap<String, ([String; 2], Operation)>,
}

impl Input {
    fn resolve(&mut self, a: &str) -> Option<bool> {
        if let Some(val) = self.resolved.get(a) {
            return Some(*val);
        }

        let (dep, op) = self.dependencies.get(a)?.clone();

        let [lhs, rhs] = &dep;
        let aval = self.resolve(lhs)?;
        let bval = self.resolve(rhs)?;
        let res = op.apply(aval, bval);

        self.resolved.insert(a.to_string(), res);

        Some(res)
    }
}

impl IsInput for Input {
    fn parse(s: &str) -> Self {
        let mut resolved = HashMap::new();
        let mut dependencies = HashMap::new();
        let parts = s.split("\n\n").collect_vec();

        Seperated::newline(FuncParser::new(|s| {
            let parts = s.split(": ").collect_vec();
            resolved.insert(parts[0].to_string(), parts[1] == "1");
        }))
        .parse(parts[0]);

        Seperated::newline(FuncParser::new(|s| {
            let parts = s.split(" -> ").collect_vec();
            let inparts = parts[0].split(" ").collect_vec();

            let lhs = inparts[0];
            let op = ty_parser!(Operation).parse(inparts[1]);
            let rhs = inparts[2];
            let res = parts[1];

            let deps = [lhs.to_string(), rhs.to_string()];
            dependencies.insert(res.to_string(), (deps, op));
        }))
        .parse(parts[1]);

        Self {
            resolved,
            dependencies,
        }
    }
}

problem_parser!(ty Input);

pub fn level1(mut monitor: Input) -> usize {
    let mut b = 0;

    for i in 0u8.. {
        if i == 100 {
            break;
        }

        let s = format!("z{i:02}");
        let Some(val) = monitor.resolve(&s) else {
            break;
        };

        b |= if val { 1 } else { 0 } << i;
    }

    b
}

pub fn level2(monitor: Input) -> String {
    let mut bad = vec![];

    for (output, (ins, op)) in monitor.dependencies.iter() {
        // 1. if the output of a gate is z{nn}, then the op must be xor unless last bit
        // 2. if the output of a gate is not z{nn}, and the inputs are not both x{nn} and y{nn}, the op must not be xor
        // -- does not apply for x00 and y00 --
        // 3. if we are doing in0 xor in1, and ins are x{nn} and y{nn}, the output must be xor'ed with something else later
        // 4. similarly, if we are doing in0 and in1, the output must be or'ed with something else later

        let mut ins = ins.clone();
        ins.sort();
        let [in0, in1] = ins;

        if output.starts_with("z") && !output.ends_with("45") {
            if *op != Operation::Xor {
                bad.push(output.clone());
            }
        } else if !(in0.starts_with("x") || in1.starts_with("y")) {
            if *op == Operation::Xor {
                bad.push(output.clone());
            }
        } else if in0.starts_with("x") && in1.starts_with("y")
            || in0.starts_with("y") && in1.starts_with("x")
        {
            if in0.ends_with("00") || in1.ends_with("00") {
                continue;
            }

            let mut ops = vec![];

            for (_, (ins_l2, opb)) in monitor.dependencies.iter() {
                if ins_l2.contains(output) {
                    ops.push(*opb);
                }
            }

            if *op == Operation::Xor && !ops.contains(&Operation::Xor)
                || *op == Operation::And && !ops.contains(&Operation::Or)
            {
                bad.push(output.clone());
            }
        }
    }

    bad.sort();
    bad.iter().join(",")
}
