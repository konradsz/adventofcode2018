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

fn main() {
    let content = fs::read_to_string("input_b").unwrap();

    let mut registers = vec![0; 4];

    for line in content.lines() {
        let instruction: Vec<usize> = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        let (a, b, c) = (instruction[1], instruction[2], instruction[3]);
        match instruction[0] {
            0 => eqri(&mut registers, a, b, c),
            1 => mulr(&mut registers, a, b, c),
            2 => gtri(&mut registers, a, b, c),
            3 => gtrr(&mut registers, a, b, c),
            4 => banr(&mut registers, a, b, c),
            5 => addi(&mut registers, a, b, c),
            6 => seti(&mut registers, a, b, c),
            7 => gtir(&mut registers, a, b, c),
            8 => muli(&mut registers, a, b, c),
            9 => bori(&mut registers, a, b, c),
            10 => setr(&mut registers, a, b, c),
            11 => addr(&mut registers, a, b, c),
            12 => bani(&mut registers, a, b, c),
            13 => borr(&mut registers, a, b, c),
            14 => eqir(&mut registers, a, b, c),
            15 => eqrr(&mut registers, a, b, c),
            _ => (),
        }
    }

    println!("{}", registers[0]);
}
