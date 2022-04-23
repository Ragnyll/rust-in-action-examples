use rust_in_action::cpu::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    // initialize registers with some values
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21; mem[0x001] = 0x00; // Set opcode to 0x2100: Call the funcition at 0x100
    mem[0x002] = 0x21; mem[0x003] = 0x00; // Set opcode to 0x2100: Call the funcition at 0x100
    mem[0x004] = 0x00; mem[0x005] = 0x00; // Set opcode to 0x0000: HALT

    mem[0x100] = 0x80; mem[0x101] = 0x14; // Set opcode to 0x8014: ADD register 1s values to registers 0
    mem[0x102] = 0x80; mem[0x103] = 0x14; // Set opcode to 0x2100: Call the funcition at 0x100
    mem[0x104] = 0x00; mem[0x105] = 0xEE; // call RET
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + 10 * 2  + 10 * 2 = {}", cpu.registers[0]);
}
