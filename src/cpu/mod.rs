pub struct CPU {
    pub current_operation: u16,
    /// the two registers used to add numbers together
    pub registers: [u8; 2],
}

impl CPU {
    pub fn run(&mut self) {
        // loop {
        let opcode = self.read_opcode();

        // opcode decoding
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;
        // }

        match (c, x, y, d) {
            // dispatches execution to hardware components
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {opcode:04x}")
        }
    }

    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}
