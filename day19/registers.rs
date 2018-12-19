use std::fs;
#[derive(Debug)]
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

struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

fn execute_instruction(registers: &mut Vec<usize>, instruction: &Instruction) {
    let (a, b, c) = (instruction.a, instruction.b, instruction.c);
    match instruction.opcode {
        Opcode::Addr => addr(registers, a, b, c),
        Opcode::Addi => addi(registers, a, b, c),
        Opcode::Mulr => mulr(registers, a, b, c),
        Opcode::Muli => muli(registers, a, b, c),
        Opcode::Banr => banr(registers, a, b, c),
        Opcode::Bani => bani(registers, a, b, c),
        Opcode::Borr => borr(registers, a, b, c),
        Opcode::Bori => bori(registers, a, b, c),
        Opcode::Setr => setr(registers, a, b, c),
        Opcode::Seti => seti(registers, a, b, c),
        Opcode::Gtir => gtir(registers, a, b, c),
        Opcode::Gtri => gtri(registers, a, b, c),
        Opcode::Gtrr => gtrr(registers, a, b, c),
        Opcode::Eqir => eqir(registers, a, b, c),
        Opcode::Eqri => eqri(registers, a, b, c),
        Opcode::Eqrr => eqrr(registers, a, b, c),
    }
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut lines = content.lines();
    let mut ip_register = String::from(lines.next().unwrap());
    ip_register.retain(|c| c.is_numeric());
    let ip_register = ip_register.parse::<usize>().unwrap();

    let mut registers = vec![0; 6];
    let mut instructions = Vec::new();
    let mut ip = 0;

    for line in lines {
        let mut parts = line.split_whitespace();
        let opcode = match parts.next().unwrap() {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            _ => panic!("Unknown opcode!"),
        };

        let a = parts.next().unwrap().parse::<usize>().unwrap();
        let b = parts.next().unwrap().parse::<usize>().unwrap();
        let c = parts.next().unwrap().parse::<usize>().unwrap();

        instructions.push(Instruction { opcode, a, b, c });
    }

    while ip <= instructions.len() {
        registers[ip_register] = ip;
        execute_instruction(&mut registers, &instructions[ip]);
        ip = registers[ip_register];
        ip += 1
    }

    println!("{}", registers[0]);
}

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
