use std::fs;

fn addr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] + registers[b];
}

fn addi(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] + b;
}

fn mulr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] * registers[b];
}

fn muli(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] * b;
}

fn banr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] & registers[b];
}

fn bani(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] & b;
}

fn borr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] | registers[b];
}

fn bori(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = registers[a] | b;
}

fn setr(registers: &mut Vec<usize>, a: usize, _: usize, c: usize) {
    registers[c] = registers[a];
}

fn seti(registers: &mut Vec<usize>, a: usize, _: usize, c: usize) {
    registers[c] = a;
}

fn gtir(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = (a > registers[b]) as usize;
}

fn gtri(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = (registers[a] > b) as usize;
}

fn gtrr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = (registers[a] > registers[b]) as usize;
}

fn eqir(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = (a == registers[b]) as usize;
}

fn eqri(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = (registers[a] == b) as usize;
}

fn eqrr(registers: &mut Vec<usize>, a: usize, b: usize, c: usize) {
    registers[c] = (registers[a] == registers[b]) as usize;
}

#[derive(PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let content: Vec<&str> = content.lines().collect();
    let number_of_samples = content.len() / 4;

    let ops = vec![
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
    ];

    let mut correct_samples = 0;
    for i in 0..=number_of_samples {
        let mut before = String::from(content[i * 4]);
        before.retain(|c| c.is_numeric());
        let op: Vec<usize> = content[i * 4 + 1]
            .split_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();
        let (a, b, c) = (op[1], op[2], op[3]);
        let mut after = String::from(content[i * 4 + 2]);
        after.retain(|c| c.is_numeric());
        let after: Vec<usize> = after
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let mut matches = 0;
        for op in ops.iter() {
            let mut registers: Vec<usize> = before
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            match op {
                Opcode::Addr => addr(&mut registers, a, b, c),
                Opcode::Addi => addi(&mut registers, a, b, c),
                Opcode::Mulr => mulr(&mut registers, a, b, c),
                Opcode::Muli => muli(&mut registers, a, b, c),
                Opcode::Banr => banr(&mut registers, a, b, c),
                Opcode::Bani => bani(&mut registers, a, b, c),
                Opcode::Borr => borr(&mut registers, a, b, c),
                Opcode::Bori => bori(&mut registers, a, b, c),
                Opcode::Setr => setr(&mut registers, a, b, c),
                Opcode::Seti => seti(&mut registers, a, b, c),
                Opcode::Gtir => gtir(&mut registers, a, b, c),
                Opcode::Gtri => gtri(&mut registers, a, b, c),
                Opcode::Gtrr => gtrr(&mut registers, a, b, c),
                Opcode::Eqir => eqir(&mut registers, a, b, c),
                Opcode::Eqri => eqri(&mut registers, a, b, c),
                Opcode::Eqrr => eqrr(&mut registers, a, b, c),
            }
            if registers == after {
                matches += 1;
            }
        }
        if matches >= 3 {
            correct_samples += 1;
        }
    }

    println!("{}", correct_samples);
}
