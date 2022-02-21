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
enum Expr {
    Const(i32),
    Add(Rc<Expr>, Rc<Expr>),
    Mul(Rc<Expr>, Rc<Expr>),
    Div(Rc<Expr>, Rc<Expr>),
    Mod(Rc<Expr>, Rc<Expr>),
    Eq(Rc<Expr>, Rc<Expr>),
    Neq(Rc<Expr>, Rc<Expr>),
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
    w: Rc<Expr>,
    x: Rc<Expr>,
    y: Rc<Expr>,
    z: Rc<Expr>,
}
impl RegisterFile {
    fn new() -> Self {
        RegisterFile {
            x: Rc::new(Expr::Const(0)),
            y: Rc::new(Expr::Const(0)),
            z: Rc::new(Expr::Const(0)),
            w: Rc::new(Expr::Const(0)),
        }
    }

    fn register(&self, id: Register) -> Rc<Expr> {
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
            Operand::Constant(c) => Rc::new(Expr::Const(c)),
            Operand::Register(reg_id) => self.register(reg_id),
        };

        let expression = match (instruction.opType, &*operand1, &*operand2) {
            (OpType::Input, _, &Expr::Const(digit)) => Rc::new(Expr::InputDigit(digit)),
            (OpType::Input, _, _) => panic!("input needs digit operand"),

            (OpType::Add, &Expr::Const(0), _) => operand2,
            (OpType::Add, _, &Expr::Const(0)) => operand1,
            (OpType::Add, &Expr::Const(c1), &Expr::Const(c2)) => Rc::new(Expr::Const(c1 + c2)),
            (OpType::Add, _, _) => Rc::new(Expr::Add(operand1, operand2)),

            (OpType::Mul, &Expr::Const(0), _) => Rc::new(Expr::Const(0)),
            (OpType::Mul, _, &Expr::Const(0)) => Rc::new(Expr::Const(0)),
            
            (OpType::Mul, &Expr::Const(1), _) => operand2,
            (OpType::Mul, _, &Expr::Const(1)) => operand1,

            (OpType::Mul, &Expr::Const(c1), &Expr::Const(c2)) => Rc::new(Expr::Const(c1 * c2)),
            (OpType::Mul, _, _) => Rc::new(Expr::Mul(operand1, operand2)),

            (OpType::Div, &Expr::Const(1), _) => operand2,
            (OpType::Div, _, &Expr::Const(1)) => operand1,

            (OpType::Div, &Expr::Const(c1), &Expr::Const(c2)) => Rc::new(Expr::Const(c1 / c2)),
            (OpType::Div, _, _) => Rc::new(Expr::Div(operand1, operand2)),

            (OpType::Mod, &Expr::Const(c1), &Expr::Const(c2)) => Rc::new(Expr::Const(c1 % c2)),
            (OpType::Mod, _, _) => Rc::new(Expr::Mod(operand1, operand2)),

            (OpType::Eq, &Expr::Eq(..), &Expr::Const(c)) if c > 1 || c < 0 => {
                Rc::new(Expr::Const(0))
            }
            (OpType::Eq, Expr::Eq(op1, op2), &Expr::Const(0)) => {
                Rc::new(Expr::Neq(op1.clone(), op2.clone()))
            }
            (OpType::Eq, &Expr::Const(c1), &Expr::Const(c2)) => {
                Rc::new(Expr::Const(if c1 == c2 { 1 } else { 0 }))
            }
            (OpType::Eq, &Expr::Const(c), Expr::InputDigit(_))
            | (OpType::Eq, Expr::InputDigit(_), &Expr::Const(c))
                if c == 0 || c > 9 =>
            {
                Rc::new(Expr::Const(0))
            }
            (OpType::Eq, _, _) => Rc::new(Expr::Eq(operand1, operand2)),
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

    for (ic, instruction) in input.iter().take(36).enumerate() {
        register_file = register_file.apply(&instruction);
        println!(
            "====={ic}: {instruction:?} =====\n\nw:{}\n\nx:{}\n\ny:{}\n\nz:{}\n",
            register_file.w, register_file.x, register_file.y, register_file.z
        );
        
    }
/*    println!(
        "\n\nw:{}\n\nx:{}\n\ny:{}\n\nz:{}\n",
        register_file.w, register_file.x, register_file.y, register_file.z
    );*/
    println!( "z:{}\n", register_file.z);

    Ok(())
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{c}"),
            Expr::Add(e0, e1) => write!(f, "({} + {})", e0, e1),
            Expr::Mul(e0, e1) => write!(f, "({} * {})", e0, e1),
            Expr::Div(e0, e1) => write!(f, "({} / {})", e0, e1),
            Expr::Mod(e0, e1) => write!(f, "({} % {})", e0, e1),
            Expr::Eq(e0, e1) => write!(f, "({} == {} ? 1 : 0)", e0, e1),
            Expr::InputDigit(e0) => write!(f, "$I{}", e0),
            Expr::Neq(e0, e1) => write!(f, "({} != {} ? 1 : 0)", e0, e1),
        }
    }
}
