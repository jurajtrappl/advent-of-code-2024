#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, space0},
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};
use rayon::prelude::*;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Clone, Debug, PartialEq)]
struct Program {
    registers: Registers,
    input: Vec<u8>,
}

fn parse_register_line(input: &str) -> IResult<&str, i64> {
    preceded(
        tuple((tag("Register"), space0, alpha1, tag(":"), space0)),
        digit1,
    )(input)
    .map(|(remaining, num)| (remaining, num.parse::<i64>().unwrap()))
}

fn parse_program(input: &str) -> IResult<&str, Vec<u8>> {
    preceded(
        tuple((tag("Program:"), space0)),
        separated_list0(tag(","), digit1),
    )(input)
    .map(|(remaining, nums)| {
        (
            remaining,
            nums.iter().map(|n| n.parse::<u8>().unwrap()).collect(),
        )
    })
}

fn parse_input(input: &str) -> IResult<&str, Program> {
    let (input, a) = parse_register_line(input)?;
    let (input, _) = newline(input)?;
    let (input, b) = parse_register_line(input)?;
    let (input, _) = newline(input)?;
    let (input, c) = parse_register_line(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let (input, program_input) = parse_program(input)?;

    Ok((
        input,
        Program {
            registers: Registers { a, b, c },
            input: program_input,
        },
    ))
}

impl Program {
    fn combo_operand(&self, value: u8) -> Option<i64> {
        match value {
            0..=3 => Some(value as i64),
            4 => Some(self.registers.a),
            5 => Some(self.registers.b),
            6 => Some(self.registers.c),
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        let numerator = self.registers.a;
        let op = self.combo_operand(operand).unwrap();
        let denominator = 2_i64.pow(op as u32);
        self.registers.a = numerator / denominator;
    }

    fn bxl(&mut self, operand: u8) {
        self.registers.b ^= operand as i64;
    }

    fn bst(&mut self, operand: u8) {
        let op = self.combo_operand(operand).unwrap();
        self.registers.b = op % 8;
    }

    fn jnz(&self, operand: u8, ip: &mut usize, jumped: &mut bool) {
        if self.registers.a != 0 {
            *ip = operand as usize;
            *jumped = true;
        }
    }

    fn bxc(&mut self) {
        self.registers.b ^= self.registers.c;
    }

    fn out(&self, operand: u8, output: &mut Vec<i64>) {
        let op = self.combo_operand(operand).unwrap();
        output.push(op % 8);
    }

    fn bdv(&mut self, operand: u8) {
        let numerator = self.registers.a;
        let op = self.combo_operand(operand).unwrap();
        let denominator = 2_i64.pow(op as u32);
        self.registers.b = numerator / denominator;
    }

    fn cdv(&mut self, operand: u8) {
        let numerator = self.registers.a;
        let op = self.combo_operand(operand).unwrap();
        let denominator = 2_i64.pow(op as u32);
        self.registers.c = numerator / denominator;
    }

    fn execute(&mut self) -> Vec<i64> {
        let mut ip = 0;
        let mut output: Vec<i64> = Vec::new();

        while ip < self.input.len() {
            let op = self.input[ip];
            let operand = self.input[ip + 1];

            let mut jumped = false;

            match op {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand, &mut ip, &mut jumped),
                4 => self.bxc(),
                5 => self.out(operand, &mut output),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!(),
            }

            if !jumped {
                ip += 2;
            }
        }

        output
    }

    fn execute_with_check(&mut self) -> bool {
        let mut ip = 0;
        let mut output: Vec<i64> = Vec::with_capacity(self.input.len());

        while ip < self.input.len() {
            let op = self.input[ip];
            let operand = self.input[ip + 1];

            let mut jumped = false;

            match op {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand, &mut ip, &mut jumped),
                4 => self.bxc(),
                5 => {
                    let op = self.combo_operand(operand).unwrap();
                    let val = op % 8;
                    if output.len() > self.input.len() || (val as u8) != self.input[output.len()] {
                        return false;
                    }

                    output.push(val);
                }
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!(),
            }

            if !jumped {
                ip += 2;
            }
        }

        output.len() == self.input.len()
    }

    fn find_lowest_self_output(&self, lower_bound: i64, upper_bound: i64) -> Option<i64> {
        (lower_bound..=upper_bound)
            .into_par_iter()
            .find_first(|&a| {
                let mut prog = self.clone();
                prog.registers.a = a;
                prog.execute_with_check()
            })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let (_, mut program) = parse_input(&input).map_err(|e| format!("Error: {}", e))?;
    let program_output = program.execute();

    let fst_part_result = program_output.iter().join(",");

    let now = Instant::now();
    let snd_part_result = program.find_lowest_self_output(0, 1_000_000_000);
    match snd_part_result {
        Some(snd_part_solution) => {
            println!(
                "First part: {}, Second part: {}",
                fst_part_result, snd_part_solution
            );
        }
        None => {
            let time_elapsed = now.elapsed();

            println!("No solution. Elapsed: {:.2?}", time_elapsed);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_input, Program, Registers};
    use itertools::Itertools;

    #[test]
    fn test_parse() {
        let input = aoe::read_input_file("example_input").unwrap();
        let (_, parsed_program) = parse_input(&input)
            .map_err(|e| format!("Error: {}", e))
            .unwrap();
        assert_eq!(
            parsed_program,
            Program {
                registers: Registers { a: 729, b: 0, c: 0 },
                input: vec![0, 1, 5, 4, 3, 0]
            }
        )
    }

    #[test]
    fn test_example_fst_part() {
        let input = aoe::read_input_file("example_input").unwrap();
        let (_, mut parsed_program) = parse_input(&input)
            .map_err(|e| format!("Error: {}", e))
            .unwrap();
        let example_program_output = parsed_program.execute().iter().join(",");
        assert_eq!(example_program_output, "4,6,3,5,6,3,5,2,1,0")
    }

    #[test]
    fn test_example_snd_part() {
        let input = aoe::read_input_file("example_input1").unwrap();
        let (_, parsed_program) = parse_input(&input)
            .map_err(|e| format!("Error: {}", e))
            .unwrap();
        let lowest_a = parsed_program.find_lowest_self_output(0, 150_000).unwrap();
        assert_eq!(117440, lowest_a);
    }
}
