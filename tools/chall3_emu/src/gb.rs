#[derive(Debug)]
pub enum GameboyNamedRegister8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum GameboyNamedRegister16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug)]
pub enum GameboyRegisterFlags {
    Z = 0b1000_0000,
    N = 0b0100_0000,
    H = 0b0010_0000,
    C = 0b0001_0000,
}

#[derive(Debug)]
pub struct GameboyRegisters {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub pc: u16,
    pub sp: u16,
}

impl Default for GameboyRegisters {
    fn default() -> Self {
        GameboyRegisters {
            af: 0x01B0,
            bc: 0x0013,
            de: 0x00D8,
            hl: 0x014D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

impl GameboyRegisters {
    pub fn new() -> Self {
        GameboyRegisters::default()
    }

    pub fn get_flag(&self, flag: GameboyRegisterFlags) -> bool {
        self.af & flag as u16 != 0
    }

    pub fn set_flag(&mut self, flag: GameboyRegisterFlags, value: bool) {
        if value {
            self.af |= flag as u16;
        } else {
            self.af &= !(flag as u16);
        }
    }

    pub fn get_reg8(&self, register: &GameboyNamedRegister8) -> u8 {
        match register {
            GameboyNamedRegister8::A => (self.af >> 8) as u8,
            GameboyNamedRegister8::B => (self.bc >> 8) as u8,
            GameboyNamedRegister8::C => self.bc as u8,
            GameboyNamedRegister8::D => (self.de >> 8) as u8,
            GameboyNamedRegister8::E => self.de as u8,
            GameboyNamedRegister8::H => (self.hl >> 8) as u8,
            GameboyNamedRegister8::L => self.hl as u8,
            _ => panic!("Invalid register: {:?}", register),
        }
    }

    pub fn set_reg8(&mut self, register: &GameboyNamedRegister8, value: u8) {
        match register {
            GameboyNamedRegister8::A => self.af = (self.af & 0x00FF) | (value as u16) << 8,
            GameboyNamedRegister8::B => self.bc = (self.bc & 0x00FF) | (value as u16) << 8,
            GameboyNamedRegister8::C => self.bc = (self.bc & 0xFF00) | value as u16,
            GameboyNamedRegister8::D => self.de = (self.de & 0x00FF) | (value as u16) << 8,
            GameboyNamedRegister8::E => self.de = (self.de & 0xFF00) | value as u16,
            GameboyNamedRegister8::H => self.hl = (self.hl & 0x00FF) | (value as u16) << 8,
            GameboyNamedRegister8::L => self.hl = (self.hl & 0xFF00) | value as u16,
            _ => panic!("Invalid register: {:?}", register),
        }
    }

    pub fn get_reg16(&self, register: &GameboyNamedRegister16) -> u16 {
        match register {
            GameboyNamedRegister16::BC => self.bc,
            GameboyNamedRegister16::DE => self.de,
            GameboyNamedRegister16::HL => self.hl,
            GameboyNamedRegister16::SP => self.sp,
            GameboyNamedRegister16::PC => self.pc,
            _ => panic!("Invalid register: {:?}", register),
        }
    }

    pub fn set_reg16(&mut self, register: &GameboyNamedRegister16, value: u16) {
        match register {
            GameboyNamedRegister16::BC => self.bc = value,
            GameboyNamedRegister16::DE => self.de = value,
            GameboyNamedRegister16::HL => self.hl = value,
            GameboyNamedRegister16::SP => self.sp = value,
            GameboyNamedRegister16::PC => self.pc = value,
            _ => panic!("Invalid register: {:?}", register),
        }
    }
}

#[derive(Debug)]
pub enum GameboyInstructionPointerOp {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub enum GameboyInstructionOperand {
    Register8(GameboyNamedRegister8),
    Register16(GameboyNamedRegister16),
    Pointer(GameboyNamedRegister16, Option<GameboyInstructionPointerOp>),
    Immediate8(u8),
    ImmediateSigned8(i8),
    Immediate16(u16),
    Address(u16),
}

#[derive(Debug)]
pub enum GameboyInstructionFamily {
    NOP,
    JP,
    DI,
    XOR,
    LD,
    DEC,
    JR,
    CALL,
    OR,
    RET,
    LDH,
    CP,
    INC,
    AND,
    PUSH,
    POP,
    SCF,
    ADD,
    BIT,
    SWAP,
}

#[derive(Debug)]
pub enum GameboyInstructionCondition {
    NZ,
    Z,
    NC,
    C,
}

#[derive(Debug)]
pub struct GameboyInstruction {
    opcode: u8,
    mnemonic: &'static str,
    instruction_family: GameboyInstructionFamily,
    operand1: Option<GameboyInstructionOperand>,
    operand2: Option<GameboyInstructionOperand>,
    condition: Option<GameboyInstructionCondition>,
    cycles: u8,
    size: u8,
}

#[derive(Debug)]
pub struct Gameboy {
    pub registers: GameboyRegisters,
    pub memory: [u8; 0xFFFF],
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            registers: GameboyRegisters::default(),
            memory: [0; 0xFFFF],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory[0..32768].copy_from_slice(&rom);
    }

    pub fn reset(&mut self) {
        self.registers = GameboyRegisters::default();
    }

    pub fn fetch(&mut self) -> u8 {
        let opcode = self.memory[self.registers.pc as usize];
        self.registers.pc += 1;

        opcode
    }

    pub fn fetch16(&mut self) -> u16 {
        let low = self.fetch() as u16;
        let high = self.fetch() as u16;

        (high << 8) | low
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }

    pub fn write_bytes(&mut self, address: u16, bytes: &[u8]) {
        self.memory[address as usize..(address + bytes.len() as u16) as usize].copy_from_slice(bytes);
    }

    pub fn read_bytes(&mut self, address: u16, length: u16) -> Vec<u8> {
        self.memory[address as usize..(address + length) as usize].to_vec()
    }

    pub fn decode_prefix(&mut self, opcode: u8) -> GameboyInstruction {
        match opcode {
            0x37 => GameboyInstruction {
                opcode,
                mnemonic: "SWAP A",
                instruction_family: GameboyInstructionFamily::SWAP,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 2,
            },
            0x7C => GameboyInstruction {
                opcode,
                mnemonic: "BIT 7, H",
                instruction_family: GameboyInstructionFamily::BIT,
                operand1: Some(GameboyInstructionOperand::Immediate8(7)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::H)),
                condition: None,
                cycles: 8,
                size: 2,
            },
            _ => panic!("Unknown prefix opcode: {:#04X}", opcode)
        }
    }

    pub fn decode(&mut self, opcode: u8) -> GameboyInstruction {
        match opcode {
            0x00 => GameboyInstruction {
                opcode,
                mnemonic: "NOP",
                instruction_family: GameboyInstructionFamily::NOP,
                operand1: None,
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x01 => GameboyInstruction {
                opcode,
                mnemonic: "LD BC, n16",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::BC)),
                operand2: Some(GameboyInstructionOperand::Immediate16(self.fetch16())),
                condition: None,
                cycles: 12,
                size: 3,
            },
            0x03 => GameboyInstruction {
                opcode,
                mnemonic: "INC BC",
                instruction_family: GameboyInstructionFamily::INC,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::BC)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x09 => GameboyInstruction {
                opcode,
                mnemonic: "ADD HL, BC",
                instruction_family: GameboyInstructionFamily::ADD,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::HL)),
                operand2: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::BC)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x0B => GameboyInstruction {
                opcode,
                mnemonic: "DEC BC",
                instruction_family: GameboyInstructionFamily::DEC,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::BC)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x0C => GameboyInstruction {
                opcode,
                mnemonic: "INC C",
                instruction_family: GameboyInstructionFamily::INC,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x0D => GameboyInstruction {
                opcode,
                mnemonic: "DEC C",
                instruction_family: GameboyInstructionFamily::DEC,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x0E => GameboyInstruction {
                opcode,
                mnemonic: "LD C, n8",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                operand2: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                condition: None,
                cycles: 8,
                size: 2,
            },
            0x11 => GameboyInstruction {
                opcode,
                mnemonic: "LD DE, n16",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::DE)),
                operand2: Some(GameboyInstructionOperand::Immediate16(self.fetch16())),
                condition: None,
                cycles: 12,
                size: 3,
            },
            0x12 => GameboyInstruction {
                opcode,
                mnemonic: "LD (DE), A",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::DE, None)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x13 => GameboyInstruction {
                opcode,
                mnemonic: "INC DE",
                instruction_family: GameboyInstructionFamily::INC,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::DE)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x18 => GameboyInstruction {
                opcode,
                mnemonic: "JR n8",
                instruction_family: GameboyInstructionFamily::JR,
                operand1: Some(GameboyInstructionOperand::ImmediateSigned8(self.fetch() as i8)),
                operand2: None,
                condition: None,
                cycles: 12,
                size: 2,
            },
            0x1A => GameboyInstruction {
                opcode,
                mnemonic: "LD A, (DE)",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::DE, None)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x1B => GameboyInstruction {
                opcode,
                mnemonic: "DEC DE",
                instruction_family: GameboyInstructionFamily::DEC,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::DE)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x1C => GameboyInstruction {
                opcode,
                mnemonic: "INC E",
                instruction_family: GameboyInstructionFamily::INC,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::E)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x1E => GameboyInstruction {
                opcode,
                mnemonic: "LD E, n8",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::E)),
                operand2: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                condition: None,
                cycles: 8,
                size: 2,
            },
            0x20 => GameboyInstruction {
                opcode,
                mnemonic: "JR NZ, n8",
                instruction_family: GameboyInstructionFamily::JR,
                operand1: Some(GameboyInstructionOperand::ImmediateSigned8(self.fetch() as i8)),
                operand2: None,
                condition: Some(GameboyInstructionCondition::NZ),
                cycles: 8,
                size: 2,
            },
            0x21 => GameboyInstruction {
                opcode,
                mnemonic: "LD HL, n16",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::HL)),
                operand2: Some(GameboyInstructionOperand::Immediate16(self.fetch16())),
                condition: None,
                cycles: 12,
                size: 3,
            },
            0x22 => GameboyInstruction {
                opcode,
                mnemonic: "LD (HL+), A",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::HL, Some(GameboyInstructionPointerOp::Increment))),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x23 => GameboyInstruction {
                opcode,
                mnemonic: "INC HL",
                instruction_family: GameboyInstructionFamily::INC,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::HL)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x28 => GameboyInstruction {
                opcode,
                mnemonic: "JR Z, n8",
                instruction_family: GameboyInstructionFamily::JR,
                operand1: Some(GameboyInstructionOperand::ImmediateSigned8(self.fetch() as i8)),
                operand2: None,
                condition: Some(GameboyInstructionCondition::Z),
                cycles: 8,
                size: 2,
            },
            0x2A => GameboyInstruction {
                opcode,
                mnemonic: "LD A, (HL+)",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::HL, Some(GameboyInstructionPointerOp::Increment))),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x31 => GameboyInstruction {
                opcode,
                mnemonic: "LD SP, n16",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::SP)),
                operand2: Some(GameboyInstructionOperand::Immediate16(self.fetch16())),
                condition: None,
                cycles: 12,
                size: 3,
            },
            0x37 => GameboyInstruction {
                opcode,
                mnemonic: "SCF",
                instruction_family: GameboyInstructionFamily::SCF,
                operand1: None,
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x38 => GameboyInstruction {
                opcode,
                mnemonic: "JR C, n8",
                instruction_family: GameboyInstructionFamily::JR,
                operand1: Some(GameboyInstructionOperand::ImmediateSigned8(self.fetch() as i8)),
                operand2: None,
                condition: Some(GameboyInstructionCondition::C),
                cycles: 8,
                size: 2,
            },
            0x3E => GameboyInstruction {
                opcode,
                mnemonic: "LD A, n8",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                condition: None,
                cycles: 8,
                size: 2,
            },
            0x40 => GameboyInstruction {
                opcode,
                mnemonic: "LD B, B",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x45 => GameboyInstruction {
                opcode,
                mnemonic: "LD B, L",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::L)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x46 => GameboyInstruction {
                opcode,
                mnemonic: "LD B, (HL)",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                operand2: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::HL, None)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x47 => GameboyInstruction {
                opcode,
                mnemonic: "LD B, A",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x4F => GameboyInstruction {
                opcode,
                mnemonic: "LD C, A",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x54 => GameboyInstruction {
                opcode,
                mnemonic: "LD D, H",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::D)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::H)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x60 => GameboyInstruction {
                opcode,
                mnemonic: "LD H, B",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::H)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x66 => GameboyInstruction {
                opcode,
                mnemonic: "LD H, (HL)",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::H)),
                operand2: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::HL, None)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0x67 => GameboyInstruction {
                opcode,
                mnemonic: "LD H, A",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::H)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x69 => GameboyInstruction {
                opcode,
                mnemonic: "LD L, C",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::L)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x6F => GameboyInstruction {
                opcode,
                mnemonic: "LD L, A",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::L)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x79 => GameboyInstruction {
                opcode,
                mnemonic: "LD A, C",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x7B => GameboyInstruction {
                opcode,
                mnemonic: "LD A, E",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::E)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x7C => GameboyInstruction {
                opcode,
                mnemonic: "LD A, H",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::H)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x7D => GameboyInstruction {
                opcode,
                mnemonic: "LD A, L",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::L)),
                condition: None,
                cycles: 4,
                size: 1,
            },
            0x7E => GameboyInstruction {
                opcode,
                mnemonic: "LD A, (HL)",
                instruction_family: GameboyInstructionFamily::LD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::HL, None)),
                condition: None,
                cycles: 8,
                size: 1,
            },
            0xA7 => GameboyInstruction {
                opcode,
                mnemonic: "AND A",
                instruction_family: GameboyInstructionFamily::AND,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xAF => GameboyInstruction {
                opcode,
                mnemonic: "XOR A",
                instruction_family: GameboyInstructionFamily::XOR,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xB0 => GameboyInstruction {
                opcode,
                mnemonic: "OR B",
                instruction_family: GameboyInstructionFamily::OR,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xB7 => GameboyInstruction {
                opcode,
                mnemonic: "OR A",
                instruction_family: GameboyInstructionFamily::OR,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xB8 => GameboyInstruction {
                opcode,
                mnemonic: "CP B",
                instruction_family: GameboyInstructionFamily::CP,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::B)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xB9 => GameboyInstruction {
                opcode,
                mnemonic: "CP C",
                instruction_family: GameboyInstructionFamily::CP,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::C)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xBE => GameboyInstruction {
                opcode,
                mnemonic: "CP (HL)",
                instruction_family: GameboyInstructionFamily::CP,
                operand1: Some(GameboyInstructionOperand::Pointer(GameboyNamedRegister16::HL, None)),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 1,
            },
            0xC1 => GameboyInstruction {
                opcode,
                mnemonic: "POP BC",
                instruction_family: GameboyInstructionFamily::POP,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::BC)),
                operand2: None,
                condition: None,
                cycles: 12,
                size: 1,
            },
            0xC3 => GameboyInstruction {
                opcode,
                mnemonic: "JP",
                instruction_family: GameboyInstructionFamily::JP,
                operand1: Some(GameboyInstructionOperand::Address(self.fetch16())),
                operand2: None,
                condition: None,
                cycles: 16,
                size: 3,
            },
            0xC5 => GameboyInstruction {
                opcode,
                mnemonic: "PUSH BC",
                instruction_family: GameboyInstructionFamily::PUSH,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::BC)),
                operand2: None,
                condition: None,
                cycles: 16,
                size: 1,
            },
            0xC6 => GameboyInstruction {
                opcode,
                mnemonic: "ADD A, n8",
                instruction_family: GameboyInstructionFamily::ADD,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                condition: None,
                cycles: 8,
                size: 2,
            },
            0xC8 => GameboyInstruction {
                opcode,
                mnemonic: "RET Z",
                instruction_family: GameboyInstructionFamily::RET,
                operand1: None,
                operand2: None,
                condition: Some(GameboyInstructionCondition::Z),
                cycles: 20,
                size: 1,
            },
            0xC9 => GameboyInstruction {
                opcode,
                mnemonic: "RET",
                instruction_family: GameboyInstructionFamily::RET,
                operand1: None,
                operand2: None,
                condition: None,
                cycles: 16,
                size: 1,
            },
            0xCA => GameboyInstruction {
                opcode,
                mnemonic: "JP Z, n16",
                instruction_family: GameboyInstructionFamily::JP,
                operand1: Some(GameboyInstructionOperand::Address(self.fetch16())),
                operand2: None,
                condition: Some(GameboyInstructionCondition::Z),
                cycles: 16,
                size: 3,
            },
            0xCB => {
                let prefix_opcode = self.fetch();
                self.decode_prefix(prefix_opcode)
            },
            0xCD => GameboyInstruction {
                opcode,
                mnemonic: "CALL",
                instruction_family: GameboyInstructionFamily::CALL,
                operand1: Some(GameboyInstructionOperand::Address(self.fetch16())),
                operand2: None,
                condition: None,
                cycles: 24,
                size: 3,
            },
            0xD1 => GameboyInstruction {
                opcode,
                mnemonic: "POP DE",
                instruction_family: GameboyInstructionFamily::POP,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::DE)),
                operand2: None,
                condition: None,
                cycles: 12,
                size: 1,
            },
            0xD5 => GameboyInstruction {
                opcode,
                mnemonic: "PUSH DE",
                instruction_family: GameboyInstructionFamily::PUSH,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::DE)),
                operand2: None,
                condition: None,
                cycles: 16,
                size: 1,
            },
            0xD8 => GameboyInstruction {
                opcode,
                mnemonic: "RET C",
                instruction_family: GameboyInstructionFamily::RET,
                operand1: None,
                operand2: None,
                condition: Some(GameboyInstructionCondition::C),
                cycles: 20,
                size: 1,
            },
            0xDA => GameboyInstruction {
                opcode,
                mnemonic: "JP C, n16",
                instruction_family: GameboyInstructionFamily::JP,
                operand1: Some(GameboyInstructionOperand::Address(self.fetch16())),
                operand2: None,
                condition: Some(GameboyInstructionCondition::C),
                cycles: 16,
                size: 3,
            },
            0xE0 => GameboyInstruction {
                opcode,
                mnemonic: "LDH (n8), A",
                instruction_family: GameboyInstructionFamily::LDH,
                operand1: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                operand2: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                condition: None,
                cycles: 12,
                size: 2,
            },
            0xE1 => GameboyInstruction {
                opcode,
                mnemonic: "POP HL",
                instruction_family: GameboyInstructionFamily::POP,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::HL)),
                operand2: None,
                condition: None,
                cycles: 12,
                size: 1,
            },
            0xE5 => GameboyInstruction {
                opcode,
                mnemonic: "PUSH HL",
                instruction_family: GameboyInstructionFamily::PUSH,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::HL)),
                operand2: None,
                condition: None,
                cycles: 16,
                size: 1,
            },
            0xE9 => GameboyInstruction {
                opcode,
                mnemonic: "JP HL",
                instruction_family: GameboyInstructionFamily::JP,
                operand1: Some(GameboyInstructionOperand::Register16(GameboyNamedRegister16::HL)),
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xF0 => GameboyInstruction {
                opcode,
                mnemonic: "LDH A, (n8)",
                instruction_family: GameboyInstructionFamily::LDH,
                operand1: Some(GameboyInstructionOperand::Register8(GameboyNamedRegister8::A)),
                operand2: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                condition: None,
                cycles: 12,
                size: 2,
            },
            0xF3 => GameboyInstruction {
                opcode,
                mnemonic: "DI",
                instruction_family: GameboyInstructionFamily::DI,
                operand1: None,
                operand2: None,
                condition: None,
                cycles: 4,
                size: 1,
            },
            0xFE => GameboyInstruction {
                opcode,
                mnemonic: "CP n8",
                instruction_family: GameboyInstructionFamily::CP,
                operand1: Some(GameboyInstructionOperand::Immediate8(self.fetch())),
                operand2: None,
                condition: None,
                cycles: 8,
                size: 2,
            },
            _ => panic!("Unknown opcode: {:#04X}", opcode)
        }
    }

    fn check_condition(&self, condition: Option<GameboyInstructionCondition>) -> bool {
        match condition {
            Some(GameboyInstructionCondition::NZ) => !self.registers.get_flag(GameboyRegisterFlags::Z),
            Some(GameboyInstructionCondition::Z) => self.registers.get_flag(GameboyRegisterFlags::Z),
            Some(GameboyInstructionCondition::NC) => !self.registers.get_flag(GameboyRegisterFlags::C),
            Some(GameboyInstructionCondition::C) => self.registers.get_flag(GameboyRegisterFlags::C),
            None => true,
        }
    }

    pub fn execute(&mut self, instruction: GameboyInstruction) {
        if !self.check_condition(instruction.condition) {
            return;
        }

        match instruction.instruction_family {
            GameboyInstructionFamily::NOP => {
                // NOP
                // println!("No operation");
                panic!();
            },
            GameboyInstructionFamily::JP => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Address(address)) => {
                        // JP n16
                        self.registers.pc = address;
                    },
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        // JP r16
                        let address = self.registers.get_reg16(&register);
                        self.registers.pc = address;
                    },
                    _ => panic!("Invalid operand for JP instruction"),
                }
            },
            GameboyInstructionFamily::DI => {
                // DI
                // println!("Interrupts disabled");
            },
            GameboyInstructionFamily::XOR => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // XOR A, r8
                        let value = self.registers.get_reg8(&register);
                        let result = self.registers.get_reg8(&GameboyNamedRegister8::A) ^ value;
                        self.registers.set_reg8(&GameboyNamedRegister8::A, result);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, false);
                        self.registers.set_flag(GameboyRegisterFlags::H, false);
                        self.registers.set_flag(GameboyRegisterFlags::C, false);
                    },
                    _ => panic!("Invalid operand for XOR instruction"),
                }
            },
            GameboyInstructionFamily::LD => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Immediate8(value)) => {
                                // LD r8, n8
                                self.registers.set_reg8(&register, value);
                            },
                            Some(GameboyInstructionOperand::Register8(register2)) => {
                                // LD r8, r8
                                let value = self.registers.get_reg8(&register2);
                                self.registers.set_reg8(&register, value);
                            },
                            Some(GameboyInstructionOperand::Pointer(register2, pointer_op)) => {
                                // LD r8, (r16)
                                let address = self.registers.get_reg16(&register2);
                                let value = self.memory[address as usize];
                                self.registers.set_reg8(&register, value);

                                if let Some(operation) = pointer_op {
                                    match operation {
                                        GameboyInstructionPointerOp::Increment => {
                                            self.registers.set_reg16(&register2, address + 1);
                                        },
                                        GameboyInstructionPointerOp::Decrement => {
                                            self.registers.set_reg16(&register2, address - 1);
                                        },
                                    }
                                }
                            },
                            _ => panic!("Invalid operand for LD instruction"),
                        }
                    },
                    Some(GameboyInstructionOperand::Pointer(register, pointer_op)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Register8(register2)) => {
                                // LD (r16), r8
                                let address = self.registers.get_reg16(&register);
                                let value = self.registers.get_reg8(&register2);
                                self.memory[address as usize] = value;

                                if let Some(operation) = pointer_op {
                                    match operation {
                                        GameboyInstructionPointerOp::Increment => {
                                            self.registers.set_reg16(&register, address + 1);
                                        },
                                        GameboyInstructionPointerOp::Decrement => {
                                            self.registers.set_reg16(&register, address - 1);
                                        },
                                    }
                                }
                            },
                            Some(GameboyInstructionOperand::Register16(register2)) => {
                                // LD (r16), r16
                                let address = self.registers.get_reg16(&register);
                                let value = self.registers.get_reg16(&register2);
                                self.memory[address as usize] = (value >> 8) as u8;
                                self.memory[address as usize + 1] = value as u8;

                                if let Some(operation) = pointer_op {
                                    match operation {
                                        GameboyInstructionPointerOp::Increment => {
                                            self.registers.set_reg16(&register, address + 2);
                                        },
                                        GameboyInstructionPointerOp::Decrement => {
                                            self.registers.set_reg16(&register, address - 2);
                                        },
                                    }
                                }
                            },
                            _ => panic!("Invalid operand for LD instruction"),
                        }
                    }
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Immediate16(value)) => {
                                // LD r16, n16
                                self.registers.set_reg16(&register, value);
                            },
                            Some(GameboyInstructionOperand::Pointer(register2, pointer_op)) => {
                                // LD r16, (r16)
                                let address = self.registers.get_reg16(&register2);
                                let value = (self.memory[address as usize + 1] as u16) << 8 | self.memory[address as usize] as u16;
                                self.registers.set_reg16(&register, value);

                                if let Some(operation) = pointer_op {
                                    match operation {
                                        GameboyInstructionPointerOp::Increment => {
                                            self.registers.set_reg16(&register2, address + 2);
                                        },
                                        GameboyInstructionPointerOp::Decrement => {
                                            self.registers.set_reg16(&register2, address - 2);
                                        },
                                    }
                                }
                            }
                            _ => panic!("Invalid operand for LD instruction"),
                        }
                    },
                    _ => panic!("Invalid operand for LD instruction"),
                }
            },
            GameboyInstructionFamily::DEC => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // DEC r8
                        let value = self.registers.get_reg8(&register);
                        self.registers.set_reg8(&register, value.wrapping_sub(1));

                        self.registers.set_flag(GameboyRegisterFlags::Z, self.registers.get_reg8(&register) == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, true);
                        self.registers.set_flag(GameboyRegisterFlags::H, (value & 0x0F) == 0);
                    },
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        // DEC r16
                        let value = self.registers.get_reg16(&register);
                        self.registers.set_reg16(&register, value.wrapping_sub(1));
                    },
                    _ => panic!("Invalid operand for DEC instruction"),
                }
            },
            GameboyInstructionFamily::JR => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::ImmediateSigned8(offset)) => {
                        // JR n8
                        self.registers.pc = (self.registers.pc as i16 + offset as i16) as u16;
                    },
                    _ => panic!("Invalid operand for JR instruction"),
                }
            },
            GameboyInstructionFamily::CALL => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Address(address)) => {
                        // CALL n16
                        let return_address = self.registers.pc;
                        self.registers.sp -= 2;
                        self.memory[self.registers.sp as usize] = return_address as u8;
                        self.memory[self.registers.sp as usize + 1] = (return_address >> 8) as u8;
                        self.registers.pc = address;
                    },
                    _ => panic!("Invalid operand for CALL instruction"),
                }
            },
            GameboyInstructionFamily::OR => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // OR A, r8
                        let value = self.registers.get_reg8(&register);
                        let result = self.registers.get_reg8(&GameboyNamedRegister8::A) | value;
                        self.registers.set_reg8(&GameboyNamedRegister8::A, result);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, false);
                        self.registers.set_flag(GameboyRegisterFlags::H, false);
                        self.registers.set_flag(GameboyRegisterFlags::C, false);
                    },
                    _ => panic!("Invalid operand for OR instruction"),
                }
            },
            GameboyInstructionFamily::RET => {
                // RET
                let low = self.memory[self.registers.sp as usize];
                let high = self.memory[self.registers.sp as usize + 1];
                self.registers.sp += 2;
                self.registers.pc = (high as u16) << 8 | low as u16;
            },
            GameboyInstructionFamily::LDH => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Immediate8(offset)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Register8(register)) => {
                                // LDH (n8), A
                                self.memory[0xFF00 + offset as usize] = self.registers.get_reg8(&register);
                            },
                            _ => panic!("Invalid operand for LDH instruction"),
                        }
                    },
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Immediate8(offset)) => {
                                // LDH A, (n8)
                                self.registers.set_reg8(&register, self.memory[0xFF00 + offset as usize]);
                            },
                            _ => panic!("Invalid operand for LDH instruction"),
                        }
                    },
                    _ => panic!("Invalid operand for LDH instruction"),
                }
            },
            GameboyInstructionFamily::CP => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Immediate8(value)) => {
                        // CP n8
                        let a = self.registers.get_reg8(&GameboyNamedRegister8::A);
                        let result = a.wrapping_sub(value);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, true);
                        self.registers.set_flag(GameboyRegisterFlags::H, (a & 0x0F) < (value & 0x0F));
                        self.registers.set_flag(GameboyRegisterFlags::C, a < value);
                    },
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // CP r8
                        let a = self.registers.get_reg8(&GameboyNamedRegister8::A);
                        let value = self.registers.get_reg8(&register);
                        let result = a.wrapping_sub(value);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, true);
                        self.registers.set_flag(GameboyRegisterFlags::H, (a & 0x0F) < (value & 0x0F));
                        self.registers.set_flag(GameboyRegisterFlags::C, a < value);
                    },
                    Some(GameboyInstructionOperand::Pointer(register, pointer_op)) => {
                        // CP (r16)
                        let address = self.registers.get_reg16(&register);
                        let value = self.memory[address as usize];
                        let a = self.registers.get_reg8(&GameboyNamedRegister8::A);
                        let result = a.wrapping_sub(value);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, true);
                        self.registers.set_flag(GameboyRegisterFlags::H, (a & 0x0F) < (value & 0x0F));
                        self.registers.set_flag(GameboyRegisterFlags::C, a < value);

                        if let Some(operation) = pointer_op {
                            match operation {
                                GameboyInstructionPointerOp::Increment => {
                                    self.registers.set_reg16(&register, address + 1);
                                },
                                GameboyInstructionPointerOp::Decrement => {
                                    self.registers.set_reg16(&register, address - 1);
                                },
                            }
                        }
                    }
                    _ => panic!("Invalid operand for CP instruction"),
                }
            },
            GameboyInstructionFamily::INC => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // INC r8
                        let value = self.registers.get_reg8(&register);
                        self.registers.set_reg8(&register, value.wrapping_add(1));

                        self.registers.set_flag(GameboyRegisterFlags::Z, self.registers.get_reg8(&register) == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, false);
                        self.registers.set_flag(GameboyRegisterFlags::H, (value & 0x0F) == 0x0F);
                    },
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        // INC r16
                        let value = self.registers.get_reg16(&register);
                        self.registers.set_reg16(&register, value.wrapping_add(1));
                    },
                    _ => panic!("Invalid operand for INC instruction"),
                }
            },
            GameboyInstructionFamily::AND => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // AND A, r8
                        let value = self.registers.get_reg8(&register);
                        let result = self.registers.get_reg8(&GameboyNamedRegister8::A) & value;
                        self.registers.set_reg8(&GameboyNamedRegister8::A, result);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, false);
                        self.registers.set_flag(GameboyRegisterFlags::H, true);
                        self.registers.set_flag(GameboyRegisterFlags::C, false);
                    },
                    _ => panic!("Invalid operand for AND instruction"),
                }
            },
            GameboyInstructionFamily::PUSH => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        // PUSH r16
                        let value = self.registers.get_reg16(&register);
                        self.registers.sp -= 2;
                        self.memory[self.registers.sp as usize] = value as u8;
                        self.memory[self.registers.sp as usize + 1] = (value >> 8) as u8;
                    },
                    _ => panic!("Invalid operand for PUSH instruction"),
                }
            },
            GameboyInstructionFamily::POP => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        // POP r16
                        let low = self.memory[self.registers.sp as usize];
                        let high = self.memory[self.registers.sp as usize + 1];
                        self.registers.sp += 2;
                        self.registers.set_reg16(&register, (high as u16) << 8 | low as u16);
                    },
                    _ => panic!("Invalid operand for POP instruction"),
                }
            },
            GameboyInstructionFamily::SCF => {
                // SCF
                self.registers.set_flag(GameboyRegisterFlags::N, false);
                self.registers.set_flag(GameboyRegisterFlags::H, false);
                self.registers.set_flag(GameboyRegisterFlags::C, true);
            },
            GameboyInstructionFamily::ADD => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register16(register)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Register16(register2)) => {
                                // ADD r16, r16
                                let value1 = self.registers.get_reg16(&register);
                                let value2 = self.registers.get_reg16(&register2);
                                let result = value1.wrapping_add(value2);

                                self.registers.set_flag(GameboyRegisterFlags::N, false);
                                self.registers.set_flag(GameboyRegisterFlags::H, (value1 & 0x0FFF) + (value2 & 0x0FFF) > 0x0FFF);
                                self.registers.set_flag(GameboyRegisterFlags::C, value1 > 0xFFFF - value2);

                                self.registers.set_reg16(&register, result);
                            },
                            _ => panic!("Invalid operand for ADD instruction"),
                        }
                    },
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Immediate8(value)) => {
                                // ADD A, n8
                                let a = self.registers.get_reg8(&GameboyNamedRegister8::A);
                                let result = a.wrapping_add(value);

                                self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                                self.registers.set_flag(GameboyRegisterFlags::N, false);
                                self.registers.set_flag(GameboyRegisterFlags::H, (a & 0x0F) + (value & 0x0F) > 0x0F);
                                self.registers.set_flag(GameboyRegisterFlags::C, a > 0xFF - value);

                                self.registers.set_reg8(&GameboyNamedRegister8::A, result);
                            },
                            _ => panic!("Invalid operand for ADD instruction"),
                        }
                    },
                    _ => panic!("Invalid operand for ADD instruction"),
                }
            },
            GameboyInstructionFamily::BIT => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Immediate8(bit)) => {
                        match instruction.operand2 {
                            Some(GameboyInstructionOperand::Register8(register)) => {
                                // BIT n, r8
                                let value = self.registers.get_reg8(&register);
                                let result = value & (1 << bit) != 0;

                                self.registers.set_flag(GameboyRegisterFlags::Z, !result);
                                self.registers.set_flag(GameboyRegisterFlags::N, false);
                                self.registers.set_flag(GameboyRegisterFlags::H, true);
                            },
                            _ => panic!("Invalid operand for BIT instruction"),
                        }
                    },
                    _ => panic!("Invalid operand for BIT instruction"),
                }
            },
            GameboyInstructionFamily::SWAP => {
                match instruction.operand1 {
                    Some(GameboyInstructionOperand::Register8(register)) => {
                        // SWAP r8
                        let value = self.registers.get_reg8(&register);
                        let result = (value << 4) | (value >> 4);

                        self.registers.set_flag(GameboyRegisterFlags::Z, result == 0);
                        self.registers.set_flag(GameboyRegisterFlags::N, false);
                        self.registers.set_flag(GameboyRegisterFlags::H, false);
                        self.registers.set_flag(GameboyRegisterFlags::C, false);

                        self.registers.set_reg8(&register, result);
                    },
                    _ => panic!("Invalid operand for SWAP instruction"),
                }
            },
            _ => panic!("Unknown instruction family: {:?}", instruction.instruction_family),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch();
        let instruction = self.decode(opcode);
        // println!("{:#06X}: {:#04X} {}", self.registers.pc - 1, opcode, instruction.mnemonic);
        self.execute(instruction);
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
}