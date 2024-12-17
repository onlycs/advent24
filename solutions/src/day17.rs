use std::{collections::HashSet, mem};

use itertools::Itertools;
use libadvent::{IsInput, Parser};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn parse_operand(&self, op: u8) -> Operand {
        match self {
            Self::Bxl | Self::Jnz | Self::Bxc => Operand::Lit(op as u64),
            Self::Adv | Self::Bst | Self::Out | Self::Bdv | Self::Cdv => Operand::parse_combo(op),
        }
    }
}

impl Instruction {
    fn parse(num: u8) -> Self {
        assert!(num <= 7, "Instruction is > 7");
        unsafe { mem::transmute(num) } // should be fine
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operand {
    Lit(u64),   // 0..=3
    Reg(usize), // 0..3
}

impl Operand {
    fn parse_combo(num: u8) -> Self {
        assert!(num <= 6, "Instruction is > 6");

        match num {
            0..=3 => Self::Lit(num as u64),
            _ => Self::Reg(num as usize - 4),
        }
    }

    fn into_num(self, regs: [u64; 3]) -> u64 {
        match self {
            Self::Lit(n) => n,
            Self::Reg(r) => regs[r],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VirtualMachine {
    registers: [u64; 3],
    prgm: Vec<u8>,
    ptr: usize,
    out: Vec<u8>,
}

impl VirtualMachine {
    pub fn execute(&mut self) -> bool {
        if self.ptr + 1 >= self.prgm.len() {
            return false;
        }

        let op = self.prgm[self.ptr];
        let operand = self.prgm[self.ptr + 1];

        let op = Instruction::parse(op);
        let operand = op.parse_operand(operand);
        let operand = operand.into_num(self.registers);

        let mut jmp = self.ptr + 2;

        match op {
            Instruction::Adv => self.registers[0] /= 2u64.pow(operand as u32),
            Instruction::Bdv => self.registers[1] = self.registers[0] / 2u64.pow(operand as u32),
            Instruction::Cdv => self.registers[2] = self.registers[0] / 2u64.pow(operand as u32),
            Instruction::Bxl => self.registers[1] ^= operand,
            Instruction::Bst => self.registers[1] = operand % 8,
            Instruction::Jnz if self.registers[0] == 0 => {} // no-op
            Instruction::Jnz => jmp = operand as usize,
            Instruction::Bxc => self.registers[1] ^= self.registers[2],
            Instruction::Out => self.out.push((operand % 8) as u8),
        }

        self.ptr = jmp;

        true
    }

    pub fn reverse_engineer(&mut self) -> u64 {
        // a ends with zero otherwise we jnz
        let mut next = vec![0];

        // save for later
        let input_a = self.registers[0];

        /*
        this is what esoprogram does

        1. b = a mod 8
        2. b = b xor 3
        3. c = a / 2 ^ b
        4. b = b xor c
        5. b = b xor 3
        6. a = a / 2 ^ 3

        print b mod 8
        jnz a
        */

        for value in self.prgm.clone().into_iter().rev() {
            let mut prevs = vec![];

            for a_next in &next {
                // we are trying to find some A such that
                // following the above steps outputs value
                // since we have the "next" a, we know that
                // current a int-div by 8 will give us the next a
                //
                // a_cur = (a_next * 8) + k (for k in 0..8)

                for k in 0..8 {
                    // if a starts at zero then we are basically
                    // running this function over again which is
                    // weird
                    if *a_next == 0 && k == 0 {
                        continue;
                    }

                    // we can piggyback off of the fact that
                    // we already have an interpreter
                    // so just set the registers and run it
                    let a_cur = (a_next * 8) + k;
                    self.registers[0] = a_cur;

                    // run the interpreter
                    while self.out.is_empty() {
                        self.execute();
                    }

                    // if the output is the same as the value
                    // then we have found the correct A
                    // (there may be multiple, idk, so save them all)
                    if self.out[0] == value {
                        prevs.push(a_cur);
                    }

                    // clean up after ourselves
                    self.registers[0] = input_a;
                    self.registers[1] = 0;
                    self.registers[2] = 0;
                    self.out = vec![];
                    self.ptr = 0;
                }
            }

            next = prevs;
        }

        next.into_iter().min().unwrap()
    }

    pub fn run(&mut self) {
        while self.execute() {}
    }
}

impl IsInput for VirtualMachine {
    fn parse(s: &str) -> Self {
        let split = s.split("\n\n").collect_vec();
        let mut registers = [0; 3];

        split[0].lines().enumerate().for_each(|(i, l)| {
            registers[i] = ty_parser!(u64).parse(&l["Register A: ".len()..]);
        });

        let prgm = split[1]["Program: ".len()..]
            .split(",")
            .map(|s| ty_parser!(u8).parse(s.trim()));

        Self {
            registers,
            prgm: prgm.collect_vec(),
            ptr: 0,
            out: Vec::new(),
        }
    }
}

problem_parser!(ty VirtualMachine);

pub fn level1(mut vm: VirtualMachine) -> String {
    println!("{:#?}", vm);
    vm.run();
    vm.out.into_iter().join(",")
}

pub fn level2(mut vm: VirtualMachine) -> u64 {
    vm.reverse_engineer()
}
