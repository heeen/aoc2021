use itertools::Itertools;
use std::{error::Error, fs};

#[derive(Debug)]
enum Register {
    x,
    y,
    z,
    w,
}

#[derive(Debug)]
enum Operand {
    Register(Register),
    Constant(i32),
}

#[derive(Debug)]
enum OpType {
    Input,
    Add,
    Mul,
    Div,
    Mod,
    Eq,
}

#[derive(Debug)]
struct Instruction {
    opType: OpType,
    register: Register,
    operand: Operand,
}

#[derive(Debug)]
enum Expression {
    Constant(i32),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    InputDigit(i32),
}

impl Register {
    fn parse(s: &str) -> Result<Self, ()> {
        match s {
            "w" => Ok(Register::w),
            "x" => Ok(Register::x),
            "y" => Ok(Register::y),
            "z" => Ok(Register::z),
            _ => Err(()),
        }
    }
}

impl Operand {
    fn parse(s: &str) -> Result<Operand, ()> {
        if let Ok(r) = Register::parse(s) {
            Ok(Operand::Register(r))
        } else if let Ok(i) = s.parse() {
            Ok(Operand::Constant(i))
        } else {
            Err(())
        }
    }
}

fn parse_op(line: &str, input_count: &mut usize) -> Instruction {
    let parts = line.split_ascii_whitespace().collect_vec();
    let (opType, register, operand) = match parts[0] {
        "inp" => {
            let i = *input_count;
            *input_count += 1;
            (
                OpType::Input,
                Register::parse(parts[1]).unwrap(),
                Operand::Constant(i as i32),
            )
        }
        "add" => (
            OpType::Add,
            Register::parse(parts[1]).unwrap(),
            Operand::parse(parts[2]).unwrap(),
        ),
        "mul" => (
            OpType::Mul,
            Register::parse(parts[1]).unwrap(),
            Operand::parse(parts[2]).unwrap(),
        ),
        "div" => (
            OpType::Div,
            Register::parse(parts[1]).unwrap(),
            Operand::parse(parts[2]).unwrap(),
        ),
        "mod" => (
            OpType::Mod,
            Register::parse(parts[1]).unwrap(),
            Operand::parse(parts[2]).unwrap(),
        ),
        "eql" => (
            OpType::Eq,
            Register::parse(parts[1]).unwrap(),
            Operand::parse(parts[2]).unwrap(),
        ),
        _ => panic!("unexpected op {}", parts[0]),
    };
    Instruction {
        opType,
        register,
        operand,
    }
}

struct RegisterFile
{
    w: Box<Expression>,
    x: Box<Expression>,
    y: Box<Expression>,
    z: Box<Expression>,
}
impl RegisterFile {

    fn new()->Self {
        RegisterFile {
             w: Box::new(Expression::Constant(0)),
             x: Box::new(Expression::Constant(0)),
             y: Box::new(Expression::Constant(0)),
             z: Box::new(Expression::Constant(0)),
        }
    }

    fn set_register(&mut self, id: Register, exp: Expression) {
        *self.register(id) = Box::new(exp);
    }
    fn register (&mut self, id: Register) -> &mut Box<Expression> {
        match id{
            Register::x => &mut self.x,
            Register::y => &mut self.y,
            Register::z => &mut self.z,
            Register::w => &mut self.w,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_count = 0;
    let input = fs::read_to_string("day24/input")?
        .lines()
        .map(|l| parse_op(l, &mut input_count))
        .collect_vec();
    for i in input.iter() {
        println!("{i:?}");
    }

    let mut register_file = RegisterFile::new();

    for instruction in input {
        let target_register = register_file.register(instruction.register);

        let operand = match instruction.operand {
            Operand::Constant(c) => Box::new(Expression::Constant(c)),
            Operand::Register(reg_id) => *register_file.register(reg_id),
        };

        let expression = match instruction.opType {
            OpType::Input => Expression::InputDigit(0),
            OpType::Add => Expression::Add(*target_register, operand),
            OpType::Mul => Expression::Mul(*target_register, operand),
            OpType::Div => Expression::Div(*target_register, operand),
            OpType::Mod => Expression::Mod(*target_register, operand),
            OpType::Eq => Expression::Eq(*target_register, operand),
        };

        *target_register = Box::new(expression);
    }
    println!("{:?}", register_file.z);

    Ok(())
}
