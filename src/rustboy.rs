use std::borrow::BorrowMut;
use std::error::Error;
use std::{collections::HashMap, hash::Hash};
use std::ops::{Index, IndexMut};

pub struct CPU_Registers{
    registers: HashMap<String, u8>,
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
            flag_register:0u8
        }
    }

    pub fn get_register(&self, key:&str) -> u8{
        return *self.registers.get(&key.to_string()).expect("Register not found");
    }
    pub fn set_register(& mut self, key:&str, value: u8){
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
        self.flag_register |= 1 << shift_length;
        self.set_register("F", self.flag_register);
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