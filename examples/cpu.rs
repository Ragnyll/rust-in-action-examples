use rust_in_action::cpu::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    // initialize registers with some values
    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14; // loads opcode 0x8014 which adds register 1 to register 0
    mem[2] = 0x80; mem[3] = 0x24; // loads opcode 0x8024 which adds register 2 to register 0
    mem[4] = 0x80; mem[5] = 0x34; // loads opcode 0x8034 which adds register 3 to register 0

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
