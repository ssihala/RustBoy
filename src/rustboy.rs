use std::borrow::BorrowMut;
use std::error::Error;
use std::{collections::HashMap, hash::Hash};
use std::ops::{Index, IndexMut};

pub struct CPU_Registers{
    registers: HashMap<String, u8>,
    opcode_register_mapping: HashMap<u8, String>,
    flag_register: u8
}

impl CPU_Registers{
    pub fn new() -> CPU_Registers{
        CPU_Registers{
            registers: HashMap::from([
                (String::from("A"), 0u8),
                (String::from("B"), 0u8),
                (String::from("C"), 0u8),
                (String::from("D"), 0u8),
                (String::from("E"), 0u8),
                (String::from("F"), 0u8),
                (String::from("H"), 0u8),
                (String::from("L"), 0u8),
            ]),
            opcode_register_mapping: HashMap::from([
                (0, String::from("B")),
                (1, String::from("C")),
                (2, String::from("D")),
                (3, String::from("E")),
                (4, String::from("H")),
                (5, String::from("L")),
                (6, String::from("HL")),
                (7, String::from("A")),

            ]),
            flag_register:0u8
        }
    }

    pub fn get_register(&self, key:&str) -> u8{
        return *self.registers.get(&key.to_string()).expect("Register not found");
    }
    pub fn set_register(&mut self, key:&str, value: u8){
        self.registers.insert(key.to_string(), value);
    }
    pub fn get_joint_register(&self, hi:&str, lo:&str) -> u16{
        ((self.get_register(hi) as u16) << 8) | self.get_register(lo) as u16
    }
    pub fn set_joint_register(&mut self, hi:&str, lo:&str, value:u16){
        let hi_value = ((value & 0xFF00) >> 8) as u8;
        let lo_value = (value & 0x00FF) as u8;
        self.registers.insert(hi.to_string(), hi_value);
        self.registers.insert(lo.to_string(), lo_value);
    }

    pub fn get_flag_shift(&self, state: &str) -> i32{
        match state{
            "Z" => 7,
            "S" => 6,
            "HC" => 5, 
            "C" => 4,
            _ => panic!("Invalid flag register"),
        }
    }
    pub fn get_flag(&self, state: &str) -> u8{
        if state.is_empty(){
            return self.flag_register
        }
        let shift_length = self.get_flag_shift(state);
        (self.flag_register >> shift_length) & 1
    }
    
    pub fn set_flag(&mut self, state: &str, value: bool){
        let shift_length: i32 = self.get_flag_shift(state);

        if value{
            self.flag_register |= (1 << shift_length);
        }
        else{
            self.flag_register &= !(1 << shift_length);
        }
        
        self.set_register("F", self.flag_register);
    }

    pub fn index_to_reg(&self, register_index: u8) -> &String{
        self.opcode_register_mapping.get(&register_index).expect("Invalid register index")
    }

}

impl Index<&str> for CPU_Registers{
    type Output = u8;

    fn index(&self, index: &str) -> &Self::Output{
        self.registers.get(index).expect("Register not found")    

    }
}

impl IndexMut<&str> for CPU_Registers {
    fn index_mut(&mut self, index: &str) -> &mut u8 {
        return self.registers.get_mut(index).expect("Register not found")
    }
}

pub struct CPU{
    pub registers: CPU_Registers
}

impl CPU{
    pub fn new() -> CPU{
        CPU{
            registers: CPU_Registers::new()
        }
    }
    pub fn add(&mut self, operand: u8){
        let operand_index = self.registers.index_to_reg(operand);
        let operand_value = self.registers[operand_index];
        let (result, overflow) = self.registers["A"].overflowing_add(operand_value);
        //SET FLAGS
        self.registers.set_flag("Z", result==0);
        self.registers.set_flag("S", false);
        self.registers.set_flag("HC", ((self.registers["A"] & 0xF) + (operand_value & 0xF)) > 0xF);
        self.registers.set_flag("C", overflow);
        self.registers["A"] = result;
    }
}

#[cfg(test)]
mod tests{
    use super::* ;
    
    #[test]
    fn test_add_base(){
        let mut cpu = super::CPU::new();
        cpu.registers["A"] = 0u8;
        cpu.registers["B"] = 7u8;    
        cpu.add(0);
        assert_eq!(cpu.registers["A"], 7u8);
        assert_eq!(cpu.registers["F"], 0u8);
    }
    #[test]
    fn test_add_zero(){
        let mut cpu = super::CPU::new();
        cpu.registers["A"] = 255;
        cpu.registers["B"] = 1;
        cpu.add(0);
        assert_eq!(cpu.registers["A"], 0);
        assert_eq!(cpu.registers["F"], 0b10110000);
    }
    #[test]
    fn test_add_halfcarry(){
        let mut cpu = super::CPU::new();
        cpu.registers["A"] = 0b00001111;
        cpu.registers["B"] = 1;
        cpu.add(0);
        assert_eq!(cpu.registers["A"], 0b10000);
        assert_eq!(cpu.registers["F"], 0b00100000);
    }
    #[test]
    fn test_add_carry(){
        let mut cpu = super::CPU::new();
        cpu.registers["A"] = 0b10000000;
        cpu.registers["B"] = 0b11111111;
        cpu.add(0);
        assert_eq!(cpu.registers["A"], 0b01111111);
        assert_eq!(cpu.registers["F"], 0b00010000);
    }
}