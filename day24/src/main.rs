use core::fmt;
use itertools::Itertools;
use std::{cell::Cell, error::Error, fmt::Display, fs, mem, ops::RangeInclusive, rc::Rc};
use termion::{color, style};

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
    Constant(i64),
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
type ExpRef = Rc<ExprSolution>;

#[derive(Debug)]
struct Instruction {
    opType: OpType,
    register: Register,
    operand: Operand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExprRange {
    start: i64,
    end: i64,
}
// inclusive range
impl ExprRange {
    fn overlaps(&self, other: &Self) -> bool {
        !(self.end <= other.start || other.end <= self.start)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.overlaps(other) {
            let start = self.start.max(other.start);
            let end = self.end.min(other.end);
            Some(Self { start, end })
        } else {
            None
        }
    }
}

impl Display for ExprRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start == self.end - 1 {
            write!(f, "[{}]", self.start)
        } else {
            write!(f, "[{}..{}]", self.start, self.end - 1)
        }
    }
}

#[derive(Clone, Debug)]
struct ExprSolution {
    expr: Expr,
    range: Cell<ExprRange>,
}

impl ExprSolution {
    fn constant(c: i64) -> ExpRef {
        let expr = Expr::Const(c);
        let range = Cell::new(expr.range());
        ExpRef::new(Self { expr, range })
    }

    fn add(op1: &Rc<ExprSolution>, op2: &Rc<ExprSolution>) -> ExpRef {
        Expr::Add(op1.clone(), op2.clone()).into()
    }
    fn mul(op1: &Rc<ExprSolution>, op2: &Rc<ExprSolution>) -> ExpRef {
        Expr::Mul(op1.clone(), op2.clone()).into()
    }
    fn div(op1: &Rc<ExprSolution>, op2: &Rc<ExprSolution>) -> ExpRef {
        Expr::Div(op1.clone(), op2.clone()).into()
    }

    fn simplify(self) -> ExpRef {
        match &self.expr {
            Expr::Add(op1, op2) => match (&op1.expr, &op2.expr) {
                (_, Expr::Const(0)) => return op1.clone(),
                (Expr::Const(0), _) => return op2.clone(),
                (Expr::Const(c1), Expr::Const(c2)) => return Self::constant(c1 + c2),
                (Expr::Add(a1, a2), Expr::Const(c2)) => {
                    if let Expr::Const(c1) = a2.expr {
                        return Self::add(a1, &Self::constant(c1 + c2)).into();
                    }
                }
                //(Expr::Eq(v1, v2, trueval, falseval), _) => {
                (Expr::Eq(v1, v2, trueval, falseval), Expr::Const(_)) => {
                    let ret = Expr::Eq(
                        v1.clone(),
                        v2.clone(),
                        Self::add(trueval, op2).into(),
                        Self::add(falseval, op2).into(),
                    )
                    .into();
                    println!("pull add {op2} into eq\n    {self}\n    {ret}");
                    return ret;
                }
                //(_, Expr::Eq(v1, v2, trueval, falseval)) => {
                (Expr::Const(_), Expr::Eq(v1, v2, trueval, falseval)) => {
                    let ret = Expr::Eq(
                        v1.clone(),
                        v2.clone(),
                        Self::add(op1, trueval),
                        Self::add(op1, falseval),
                    )
                    .into();
                    println!("pull add {op1} into eq:    {self}\n    {ret}");
                    return ret;
                }
                _ => {}
            },
            Expr::Mul(op1, op2) => match (&op1.expr, &op2.expr) {
                (_, Expr::Const(0)) => return Self::constant(0),
                (Expr::Const(0), _) => return Self::constant(0),
                (_, Expr::Const(1)) => return op1.clone(),
                (Expr::Const(1), _) => return op2.clone(),
                (Expr::Const(c1), Expr::Const(c2)) => return Self::constant(c1 * c2),
                (Expr::Mul(m1, m2), Expr::Const(c2)) => {
                    if let Expr::Const(c1) = m2.expr {
                        let ret = Self::mul(m1, &Self::constant(c1 * c2));
                        println!("folding mul {self} -> {ret}");
                        return ret;
                    }
                }
                //(Expr::Eq(v1, v2, trueval, falseval), _) => {
                (Expr::Eq(v1, v2, trueval, falseval), Expr::Const(_)) => {
                    let ret = Expr::Eq(
                        v1.clone(),
                        v2.clone(),
                        Self::mul(trueval, op2),
                        Self::mul(falseval, op2),
                    )
                    .into();
                    println!("pull mul {op1} into eq {self} -> {ret}");
                    return ret;
                }
                //(_, Expr::Eq(v1, v2, trueval, falseval)) => {
                (Expr::Const(_), Expr::Eq(v1, v2, trueval, falseval)) => {
                    let ret = Expr::Eq(
                        v1.clone(),
                        v2.clone(),
                        Self::mul(op1, trueval),
                        Self::mul(op1, falseval),
                    )
                    .into();
                    println!("pull mul {op1} into eq {self} -> {ret}");
                    return ret;
                }
                _ => {}
            },
            Expr::Div(op1, op2) => match (&op1.expr, &op2.expr) {
                (_, Expr::Const(0)) => panic!("division by zero"),
                (Expr::Const(0), _) => return Self::constant(0),
                (_, Expr::Const(1)) => return op1.clone(),
                (Expr::Const(c1), Expr::Const(c2)) => return Self::constant(c1 / c2),
                (Expr::Mul(m1, m2), &Expr::Const(c2)) => {
                    if let Expr::Const(c1) = m2.expr {
                        if c1 == c2 {
                            return m1.clone();
                        } else {
                            return Self::mul(m1, &Self::constant(c1 / c2));
                        }
                    }
                }
                (_, &Expr::Const(divisor)) if divisor > op1.range.get().end => {
                    println!(
                        "op1 {op1} range {} always smaller than {divisor}, rounding to 0",
                        op1.range.get()
                    );
                    return Expr::Const(0).into();
                }
                (_, &Expr::Const(divisor)) => {
                    println!("XXX {op1} div {divisor}");
                    if let Expr::Add(a1, a2) = &op1.expr {
                        if let Expr::Mul(m1, m2) = &a1.expr {
                            if let Expr::Const(c2) = m2.expr {
                                if divisor == c2 {
                                    if a2.range.get().end < c2 {
                                        println!("XXX just {m1}");
                                        return m1.clone();
                                    }
                                }
                            }
                        }
                    }
                }
                //                (Expr::Eq(v1, v2, trueval, falseval), _) => {
                (Expr::Eq(v1, v2, trueval, falseval), Expr::Const(_)) => {
                    let ret = Expr::Eq(
                        v1.clone(),
                        v2.clone(),
                        Self::div(trueval, op2),
                        Self::div(falseval, op2),
                    )
                    .into();
                    println!("pull div {op1} into eq {self} -> {ret}");
                    return ret;
                }
                // (_, Expr::Eq(v1, v2, trueval, falseval)) => {
                (Expr::Const(_), Expr::Eq(v1, v2, trueval, falseval)) => {
                    let ret = Expr::Eq(
                        v1.clone(),
                        v2.clone(),
                        Self::div(op1, trueval),
                        Self::div(op1, falseval),
                    )
                    .into();
                    println!("pull div {op1} into eq {self} -> {ret}");
                    return ret;
                }
                _ => {}
            },
            Expr::Mod(op1, op2) => match (&op1.expr, &op2.expr) {
                (_, Expr::Const(0)) => panic!("division by zero"),
                (Expr::Const(0), _) => return Self::constant(0),
                (_, Expr::Const(1)) => return Self::constant(1),
                (Expr::Const(c1), Expr::Const(c2)) => return Self::constant(c1 % c2),
                (_, &Expr::Const(modulus)) if op1.range.get().end <= modulus => {
                    println!(
                        "op1 {op1} range {} always smaller than modulus {modulus}, noop",
                        op1.range.get()
                    );
                    return op1.clone();
                }
                (_, &Expr::Const(modulus)) => {
                    println!("XXX {op1} {} mod {modulus} ", op1.range.get());
                    if let Expr::Add(a1, a2) = &op1.expr {
                        if let Expr::Mul(_, m2) = &a1.expr {
                            if let Expr::Const(c2) = m2.expr {
                                if modulus == c2 {
                                    if a2.range.get().end < c2 {
                                        println!("XXX just {a2}");
                                        return a2.clone();
                                    } else {
                                        let ret = Expr::Mod(a2.clone(), m2.clone()).into();
                                        println!("XXX just {ret}");
                                        return ret;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            Expr::Eq(op1, op2, trueval, falseval) => {
                let r1 = op1.range.get();
                let r2 = op2.range.get();
                if !r1.overlaps(&r2) {
                    println!("Comparison never true! {op1} == {op2} -> 0");
                    return falseval.clone();
                }

                match (&op1.expr, &op2.expr) {
                    (&Expr::Const(v1), &Expr::Const(v2)) => {
                        if v1 == v2 {
                            println!("{v1} == {v2} -> {trueval}");
                            return trueval.clone();
                        } else {
                            println!("{v1} == {v2} -> {falseval}");
                            return falseval.clone();
                        }
                    }
                    (Expr::Eq(iop1, iop2, itrueval, ifalseval), &Expr::Const(0)) => {
                        let ret = Expr::Eq(
                            iop1.clone(),
                            iop2.clone(),
                            ifalseval.clone(),
                            itrueval.clone(),
                        )
                        .into();
                        println!("invert eq {self} -> {ret}");
                        return ret;
                    }
                    (Expr::Add(a1, a2), Expr::InputDigit(_)) => {
                        if let Expr::InputDigit(_) = a1.expr {
                            if let Expr::Const(c) = a2.expr {
                                println!("{self} input offset comparison can be true!");
                                let r1 = a1.range.get();
                                let r2 = op2.range.get();
                                if c > 0 {
                                    // a1 + 3 = o2
                                    a1.range.set(ExprRange {
                                        start: r1.start,
                                        end: r2.end - c,
                                    });
                                    op2.range.set(ExprRange {
                                        start: r1.start + c,
                                        end: r2.end,
                                    });
                                } else {
                                    // a1 -3 = op2
                                    a1.range.set(ExprRange {
                                        start: r1.start - c,
                                        end: r1.end,
                                    });
                                    op2.range.set(ExprRange {
                                        start: r2.start,
                                        end: r2.end + c,
                                    });
                                }
                                println!("==> {a1} + {c} = {op2} -> {trueval}");

                                return trueval.clone();
                            }
                        }
                    }
                    (_, _) => return ExpRef::new(self),
                }
            }
            _ => return ExpRef::new(self),
        }

        ExpRef::new(self)
    }
    fn is_const(&self) -> bool {
        match self.expr {
            Expr::Const(_) => true,
            _ => false,
        }
    }

    fn solve(&self, range: ExprRange) -> Vec<ExprSolution> {
        let r = self.range.get();
        if let Some(irange) = r.intersection(&range) {
            if irange.end == irange.start {
                return vec![ExprSolution {
                    expr: Expr::Const(irange.start),
                    range: Cell::new(ExprRange {
                        start: irange.start,
                        end: irange.end,
                    }),
                }];
            }
            match &self.expr {
                Expr::Const(_) => vec![self.clone()],
                Expr::Add(a1, a2) => {
                    let r1 = a1.range.get();
                    let r2 = a2.range.get();

                    let min1 = irange.start - r2.end;
                    let max1 = irange.end - r2.start;

                    let min2 = irange.start - r1.end;
                    let max2 = irange.end - r1.start;
                    println!("solve add {a1}\n+\n{a2}\n = {range}");
                    let mut solutions = Vec::new();
                    for i in min1..max1 {
                        let r1 = ExprRange {
                            start: i,
                            end: i + 1,
                        };
                        let s1 = a1.solve(r);
                    }
                    /* let a1 = ExprSolution {
                        expr: a1.expr,
                        range: Cell::new(ExprRange{ start: min1, end: max1})
                    };*/
                    solutions
                }
                Expr::Mul(m1, m2) => {
                    println!("solve mul {m1}*{m2}={range}");
                    todo!()
                }
                Expr::Div(d1, d2) => {
                    println!("solve div {d1}/{d2} = {range}");
                    todo!()
                }
                Expr::Mod(m1, m2) => {
                    println!("solve mod {m1}%{m2} = {range}");
                    todo!()
                }
                Expr::Eq(v1, v2, t, f) => {
                    println!("solve eq {v1}=={v2}?{t}:{f} = {range}");
                    todo!()
                }
                Expr::InputDigit(d) => {
                    println!("solve input {d} = {range}");
                    todo!()
                }
                _ => vec![],
            }
        } else {
            println!("no possible solution (range) {self} {r} in {range}");
            vec![]
        }
    }
}

#[derive(Debug, Clone)]
enum Expr {
    Const(i64),
    Add(ExpRef, ExpRef),
    Mul(ExpRef, ExpRef),
    Div(ExpRef, ExpRef),
    Mod(ExpRef, ExpRef),
    Eq(ExpRef, ExpRef, ExpRef, ExpRef),
    InputDigit(i64),
}

impl Expr {
    fn range(&self) -> ExprRange {
        let (start, end) = match self {
            &Expr::Const(c) => (c, c + 1),
            Expr::Add(o1, o2) => {
                let r1 = o1.range.get();
                let r2 = o2.range.get();
                (r1.start + r2.start, (r1.end + r2.end) - 1)
            }
            Expr::Mul(o1, o2) => {
                let r1 = o1.range.get();
                let r2 = o2.range.get();
                (r1.start * r2.start, (r1.end - 1) * (r2.end - 1) + 1)
            }
            Expr::Div(o1, o2) => {
                let r1 = o1.range.get();
                let r2 = o2.range.get();
                (r1.start / r2.start, (r1.end - 1) / (r2.end - 1) + 1)
            }
            Expr::Mod(o1, o2) => {
                let r2 = o2.range.get();
                (0, r2.end - 1)
            }
            Expr::Eq(_, _, trueval, falseval) => {
                let r1 = trueval.range.get();
                let r2 = falseval.range.get();
                (r1.start.min(r2.start), r1.end.max(r2.end))
            }
            Expr::InputDigit(_) => (1, 10),
        };
        ExprRange { start, end }
    }
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
                Operand::Constant(i as i64),
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
    w: ExpRef,
    x: ExpRef,
    y: ExpRef,
    z: ExpRef,
    inputs: Vec<ExpRef>,
}
impl RegisterFile {
    fn new() -> Self {
        RegisterFile {
            x: Expr::Const(0).into(),
            y: Expr::Const(0).into(),
            z: Expr::Const(0).into(),
            w: Expr::Const(0).into(),
            inputs: (0..14).map(|i| Expr::InputDigit(i).into()).collect(),
        }
    }

    fn register(&self, id: Register) -> ExpRef {
        match id {
            Register::x => &self.x,
            Register::y => &self.y,
            Register::z => &self.z,
            Register::w => &self.w,
        }
        .clone()
    }

    fn apply(self, instruction: &Instruction) -> Self {
        let ro1 = self.register(instruction.register);

        let operand1 = &*ro1;

        let ro2 = match instruction.operand {
            Operand::Constant(c) => Expr::Const(c).into(),
            Operand::Register(reg_id) => self.register(reg_id).clone(),
        };

        let operand2 = &*ro2;

        let expression = match (instruction.opType, &operand1.expr, &operand2.expr) {
            (OpType::Input, _, &Expr::Const(digit)) => self.inputs[digit as usize].clone(),
            (OpType::Input, _, _) => panic!("input needs digit operand"),
            (OpType::Add, _, _) => Expr::Add(ro1, ro2).into(),
            (OpType::Mul, _, _) => Expr::Mul(ro1, ro2).into(),
            (OpType::Div, _, _) => Expr::Div(ro1, ro2).into(),
            (OpType::Mod, _, _) => Expr::Mod(ro1, ro2).into(),
            (OpType::Eq, _, _) => {
                Expr::Eq(ro1, ro2, Expr::Const(1).into(), Expr::Const(0).into()).into()
            }
        };

        let (x, y, z, w, inputs) = (self.x, self.y, self.z, self.w, self.inputs);
        //let expression = Rc::new(*expression);
        match instruction.register {
            Register::x => RegisterFile {
                x: expression,
                y,
                z,
                w,
                inputs,
            },
            Register::y => RegisterFile {
                x,
                y: expression,
                z,
                w,
                inputs,
            },
            Register::z => RegisterFile {
                x,
                y,
                z: expression,
                w,
                inputs,
            },
            Register::w => RegisterFile {
                x,
                y,
                z,
                w: expression,
                inputs,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_count = 0;
    let input = fs::read_to_string("day24/input")?
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_op(l, &mut input_count))
        .collect_vec();
    let mut register_file = RegisterFile::new();

    for (ic, instruction) in input.iter().take(18 * 14).enumerate() {
        register_file = register_file.apply(&instruction);
        println!(
            "====={ic}: {instruction:?} =====\nx:{}\ny:{}\nz:{}\n",
            register_file.x, register_file.y, register_file.z
        );
    }
    println!(
        "\n\nw:{}\n\nx:{}\n\ny:{}\n\nz:{}\n",
        register_file.w, register_file.x, register_file.y, register_file.z
    );
    for (i, input) in register_file.inputs.iter().enumerate() {
        println!("input {i}: {input}");
    }

    println!(
        "max number: {}",
        register_file
            .inputs
            .iter()
            .map(|i| (i.range.get().end - 1).to_string())
            .collect::<String>()
    );
    println!(
        "max number: {}",
        register_file
            .inputs
            .iter()
            .map(|i| (i.range.get().start).to_string())
            .collect::<String>()
    );

    /*    let solutions = register_file.z.solve(ExprRange { start: 0, end: 1 });
        print!("solutions: {}", solutions.len());
        for s in solutions {
            println!("solution: {s}");
        }
    */
    Ok(())
}

impl Display for ExprSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expr {
            Expr::Const(c) => write!(f, "{c}"),
            Expr::Add(e0, e1) => write!(f, "({e0} + {e1})"),
            Expr::Mul(e0, e1) => write!(f, "{e0} * {e1}"),
            Expr::Div(e0, e1) => write!(f, "{e0} / {e1}"),
            Expr::Mod(e0, e1) => write!(f, "({e0} % {e1})"),
            Expr::Eq(e0, e1, trueval, falseval) => {
                write!(f, " ({e0} == {e1} ? {trueval} : {falseval})")
            }
            Expr::InputDigit(e0) => write!(f, "$I{}", e0),
        }?;
        match self.expr {
            Expr::Const(_) => Ok(()),
            _ => {
                write!(f, "{}", self.range.get())
            }
        }?;

        Ok(())
    }
}

impl From<Expr> for ExpRef {
    fn from(expr: Expr) -> Self {
        let range = Cell::new(expr.range());
        ExprSolution { expr, range }.simplify()
    }
}

impl From<RangeInclusive<i64>> for ExprRange {
    fn from(r: RangeInclusive<i64>) -> Self {
        ExprRange {
            start: *r.start(),
            end: r.end() + 1,
        }
    }
}
