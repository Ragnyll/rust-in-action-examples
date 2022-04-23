pub struct CPU {
    /// 16 8 bit registers
    /// can be registered by a single hex number 0 to F
    pub registers: [u8; 16],
    /// usually called program_counter
    pub position_in_memory: usize,
    pub memory: [u8; 0x1000],
}

impl CPU {
    pub fn run(&mut self) {
        // process all instructions
        loop {
            let opcode = self.read_opcode();
            // jump to next inestruction
            self.position_in_memory += 2;

            // opcode decoding
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

                // dispatches execution to hardware components
            match (c, x, y, d) {
                // short circuit to return when the opcode 0x0000 is encountered
                (0, 0, 0, 0) => return,
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {opcode:04x}"),
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg_x = self.registers[x as usize];
        let arg_y = self.registers[y as usize];

        let (val, overflow) = arg_x.overflowing_add(arg_y);
        self.registers[x as usize] = val;

        // the last bit on the chip 8 reprents overflow
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
