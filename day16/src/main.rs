use std::fs;
extern crate num;
#[macro_use]
extern crate num_derive;

fn main() {
    /*    let samples = [
            "D2FE28",                         // v6 id4=value payload=2021
            "38006F45291200", // v1, id6=op, length type 0=15bit, len=27, sub1 len=11, literal=10, sub2 len=16 literal=20
            "EE00D40C823060", // v7, id3=op, len id 1 = 11bit, 11 bit len = 3, sub1 len=11 lit=1, sub2 len=11 lit=2, sub3 len=11 lit=3
            "8A004A801A8002F478", // represents an operator packet (version 4) which contains an operator packet (version 1) which contains an operator packet (version 5) which contains a literal value (version 6); this packet has a version sum of 16."
            "620080001611562C8802118E34", // represents an operator packet (version 3) which contains two sub-packets; each sub-packet is an operator packet that contains two literal values. This packet has a version sum of 12.
            "C0015000016115A2E0802F182340", // has the same structure as the previous example, but the outermost packet uses a different length type ID. This packet has a version sum of 23.
            "A0016C880162017C3686B18A3D4780", // is an operator packet that contains an operator packet that contains an operator packet that contains five literal values; it has a version sum of 31.
        ];
        for (i, sample) in samples.iter().enumerate() {
            println!("--- sample {} ({})", i, sample);
            let root = parse_header(&unhex(sample), &mut 0);
        }
    */
    let contents = fs::read_to_string("day16/input").expect("could not read input");
    let root = parse_header(&unhex(&contents), &mut 0);
    println!(
        "version sum: {} result {}",
        root.version_sum(),
        root.evaluate()
    );
}

fn unhex(data: &str) -> Vec<u8> {
    data.chars()
        .fold((Vec::new(), None), |mut a, ch| {
            if let Some(nibble) = ch.to_digit(16) {
                match a.1 {
                    None => (a.0, Some(nibble)),
                    Some(prev) => {
                        a.0.push((prev * 16 + nibble) as u8);
                        (a.0, None)
                    }
                }
            } else {
                a
            }
        })
        .0
}

trait Expression {
    fn evaluate(&self) -> u64;
    fn version_sum(&self) -> u64;
}

#[derive(FromPrimitive, Debug)]
enum ExpressionType {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    Literal = 4,
    GreaterThan = 5,
    LessThan = 6,
    Equal = 7,
}

struct Literal {
    value: u64,
    version: u16,
}

struct Operator {
    op: ExpressionType,
    version: u16,
    operands: Vec<Box<dyn Expression>>,
}

impl Literal {
    pub fn parse(version: u16, data: &[u8], offset: &mut usize) -> Literal {
        let mut value = 0u64;
        loop {
            let val_part = get_bits(data, offset, 5);
            value = value * 16 + (val_part & 0b01111) as u64;
            if val_part & 0b10000 == 0 {
                break;
            }
        }
        Literal { value, version }
    }
}

impl Expression for Literal {
    fn evaluate(&self) -> u64 {
        self.value
    }

    fn version_sum(&self) -> u64 {
        self.version as u64
    }
}

impl Operator {
    pub fn parse(p_type: ExpressionType, version: u16, data: &[u8], offset: &mut usize) -> Self {
        let len_type = get_bits(data, offset, 1);
        let mut operands = Vec::new();
        if len_type == 0 {
            // length in bits
            let len_bits = get_bits(data, offset, 15) as usize;
            let end_offset = *offset + len_bits;
            while *offset < end_offset {
                operands.push(parse_header(data, offset));
            }
        } else {
            let len_packets = get_bits(data, offset, 11);
            for p in 0..len_packets {
                operands.push(parse_header(data, offset));
            }
        }
        Operator {
            op: p_type,
            version,
            operands,
        }
    }
}
impl Expression for Operator {
    fn evaluate(&self) -> u64 {
        let values = self.operands.iter().map(|e| e.evaluate());
        match self.op {
            ExpressionType::Sum => values.fold(0, |a, o| a + o),
            ExpressionType::Product => values.fold(1, |a, o| a * o),
            ExpressionType::Minimum => values.fold(u64::MAX, |a, o| a.min(o)),
            ExpressionType::Maximum => values.fold(0, |a, o| a.max(o)),
            ExpressionType::Literal => panic!("unexpected literal"),
            ExpressionType::GreaterThan => {
                let values: Vec<_> = values.collect();
                if values[0] > values[1] {
                    1
                } else {
                    0
                }
            }
            ExpressionType::LessThan => {
                let values: Vec<_> = values.collect();
                if values[0] < values[1] {
                    1
                } else {
                    0
                }
            }
            ExpressionType::Equal => {
                let values: Vec<_> = values.collect();
                if values[0] == values[1] {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn version_sum(&self) -> u64 {
        self.operands
            .iter()
            .fold(self.version as u64, |a, o| a + o.version_sum())
    }
}
fn get_bits(data: &[u8], offset: &mut usize, num_bits: usize) -> u16 {
    let start = *offset / 8;
    let end = (*offset + num_bits + 7) / 8;
    let win = &data[start..end];
    let data = match end - start {
        1 => u32::from_be_bytes([win[0], 0, 0, 0]),
        2 => u32::from_be_bytes([win[0], win[1], 0, 0]),
        3 => u32::from_be_bytes([win[0], win[1], win[2], 0]),
        4 => u32::from_be_bytes([win[0], win[1], win[2], win[3]]),
        _ => panic!("too many bits for u16"),
    };
    let rem_bits = 32 - num_bits;
    let shifted = data << *offset % 8;
    let mask = !((0b1 << rem_bits) - 1);
    *offset += num_bits;
    ((shifted & mask) >> rem_bits) as u16
}

fn parse_header(data: &[u8], offset: &mut usize) -> Box<dyn Expression> {
    let version = get_bits(data, offset, 3);
    let p_type = num::FromPrimitive::from_u16(get_bits(data, offset, 3));
    match p_type {
        Some(ExpressionType::Literal) => Box::new(Literal::parse(version, data, offset)),
        Some(et) => Box::new(Operator::parse(et, version, data, offset)),
        None => panic!("unknown operator {:?}", p_type),
    }
}
