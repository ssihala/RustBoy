mod rustboy;
use rustboy::CPU_Registers;

fn main(){
    let mut cpu:CPU_Registers = CPU_Registers::new();
    

    cpu.set_joint_register("B", "C", 515);
   
   cpu.set_flag("Z", true);
   cpu.set_flag("HC", true);
   println!("{:#08b}", cpu.get_flag(""));

}
