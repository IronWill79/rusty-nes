use std::collections::HashMap;

use crate::cpu::AddressingMode;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Self {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::None),

        // ADC
        // OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        // OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        // OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPage_X),
        // OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        // OpCode::new(0x7d, "ADC", 3, 4 /* +1 is page crossed */, AddressingMode::Absolute_X),
        // OpCode::new(0x79, "ADC", 3, 4 /* +1 is page crossed */, AddressingMode::Absolute_Y),
        // OpCode::new(0x61, "ADC", 2, 6, AddressingMode::Indirect_X),
        // OpCode::new(0x71, "ADC", 2, 5 /* +1 is page crossed */, AddressingMode::Indirect_Y),

        // INX
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::None),

        // LDA
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4 /* +1 is page crossed */, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4 /* +1 is page crossed */, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5 /* +1 is page crossed */, AddressingMode::Indirect_Y),

        // TAX
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::None)
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
