use itertools::Itertools;
use std::{error::Error, fmt::Display, fs, mem, rc::Rc};

#[derive(Debug, Copy, Clone)]
enum Register {
    x,
    y,
    z,
    w,
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    Register(Register),
    Constant(i32),
}

#[derive(Debug, Copy, Clone)]
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
    Add(Rc<Expression>, Rc<Expression>),
    Mul(Rc<Expression>, Rc<Expression>),
    Div(Rc<Expression>, Rc<Expression>),
    Mod(Rc<Expression>, Rc<Expression>),
    Eq(Rc<Expression>, Rc<Expression>),
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

#[derive(Debug)]
struct RegisterFile {
    w: Rc<Expression>,
    x: Rc<Expression>,
    y: Rc<Expression>,
    z: Rc<Expression>,
}
impl RegisterFile {
    fn new() -> Self {
        RegisterFile {
            x: Rc::new(Expression::Constant(0)),
            y: Rc::new(Expression::Constant(0)),
            z: Rc::new(Expression::Constant(0)),
            w: Rc::new(Expression::Constant(0)),
        }
    }

    fn register(&self, id: Register) -> Rc<Expression> {
        match id {
            Register::x => &self.x,
            Register::y => &self.y,
            Register::z => &self.z,
            Register::w => &self.w,
        }
        .clone()
    }

    fn apply(self, instruction: &Instruction) -> Self {
        let operand1 = self.register(instruction.register);

        let operand2 = match instruction.operand {
            Operand::Constant(c) => Rc::new(Expression::Constant(c)),
            Operand::Register(reg_id) => self.register(reg_id),
        };

        let expression = match (instruction.opType, &*operand1, &*operand2) {
            (OpType::Input, _, &Expression::Constant(digit)) => {
                Rc::new(Expression::InputDigit(digit))
            }
            (OpType::Input, _, _) => panic!("input needs digit operand"),

            (OpType::Add, &Expression::Constant(0), _) => operand2,
            (OpType::Add, _, &Expression::Constant(0)) => operand1,
            (OpType::Add, &Expression::Constant(c1), &Expression::Constant(c2)) => {
                Rc::new(Expression::Constant(c1 + c2))
            }
            (OpType::Add, _, _) => Rc::new(Expression::Add(operand1, operand2)),

            (OpType::Mul, &Expression::Constant(0), _) => Rc::new(Expression::Constant(0)),
            (OpType::Mul, _, &Expression::Constant(0)) => Rc::new(Expression::Constant(0)),

            (OpType::Mul, &Expression::Constant(c1), &Expression::Constant(c2)) => {
                Rc::new(Expression::Constant(c1 * c2))
            }
            (OpType::Mul, _, _) => Rc::new(Expression::Mul(operand1, operand2)),

            (OpType::Div, &Expression::Constant(1), _) => operand2,
            (OpType::Div, _, &Expression::Constant(1)) => operand1,

            (OpType::Div, &Expression::Constant(c1), &Expression::Constant(c2)) => {
                Rc::new(Expression::Constant(c1 / c2))
            }
            (OpType::Div, _, _) => Rc::new(Expression::Div(operand1, operand2)),

            (OpType::Mod, &Expression::Constant(c1), &Expression::Constant(c2)) => {
                Rc::new(Expression::Constant(c1 % c2))
            }
            (OpType::Mod, _, _) => Rc::new(Expression::Mod(operand1, operand2)),

            (OpType::Eq, &Expression::Constant(c1), &Expression::Constant(c2)) => {
                Rc::new(Expression::Constant(if c1 == c2 { 1 } else { 0 }))
            }
            (OpType::Eq, _, _) => Rc::new(Expression::Eq(operand1, operand2)),
        };

        let (x, y, z, w) = (self.x, self.y, self.z, self.w);
        match instruction.register {
            Register::x => RegisterFile {
                x: expression,
                y,
                z,
                w,
            },
            Register::y => RegisterFile {
                x,
                y: expression,
                z,
                w,
            },
            Register::z => RegisterFile {
                x,
                y,
                z: expression,
                w,
            },
            Register::w => RegisterFile {
                x,
                y,
                z,
                w: expression,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_count = 0;
    let input = fs::read_to_string("day24/input")?
        .lines()
        .map(|l| parse_op(l, &mut input_count))
        .collect_vec();
    /*
        for i in input.iter() {
            println!("{i:?}");
        }
    */
    let mut register_file = RegisterFile::new();

    for (ic, instruction) in input.iter().take(260).enumerate() {
        register_file = register_file.apply(&instruction);
/*        println!(
            "====={ic}: {instruction:?} =====\n\nw:{}\n\nx:{}\n\ny:{}\n\nz:{}\n",
            register_file.w, register_file.x, register_file.y, register_file.z
        );*/
    }
    println!(
        "\n\nw:{}\n\nx:{}\n\ny:{}\n\nz:{}\n",
        register_file.w, register_file.x, register_file.y, register_file.z
    );

    Ok(())
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(c) => write!(f, "{c}"),
            Expression::Add(e0, e1) => write!(f, "({} + {})", e0, e1),
            Expression::Mul(e0, e1) => write!(f, "{} * {}", e0, e1),
            Expression::Div(e0, e1) => write!(f, "{} / {}", e0, e1),
            Expression::Mod(e0, e1) => write!(f, "{} % {}", e0, e1),
            Expression::Eq(e0, e1) => write!(f, "({} == {} ? 1 : 0)", e0, e1),
            Expression::InputDigit(e0) => write!(f, "$I{}", e0),
        }
    }
}
