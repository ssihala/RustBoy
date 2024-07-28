mod rustboy;
use rustboy::CPU;

fn main(){
    let mut cpu:CPU = CPU::new();
    


    cpu.add(0);
    
//    cpu.registers.set_flag("S", true);
//    cpu.registers.set_flag("HC", true);
//    println!("{:08b}", cpu.registers["F"]);

}
