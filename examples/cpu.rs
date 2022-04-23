use rust_in_action::cpu::CPU;

fn main() {
    let mut cpu = CPU {
        current_operation: 0,
        registers: [0; 2]
    };

    // the opcode the cpu will interpert
    // - 8 signifies that the operation takes 2 registers
    // 0 maps to cpu.registers[0]
    // 1 maps to cpu.registers[1]
    // 4 indicates addition
    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}
