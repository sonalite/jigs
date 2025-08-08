use jigs::Instruction;

fn main() {
    // Example: decode and display an ADD instruction
    // add x1, x2, x3
    let instruction_word = 0x003100B3;
    let instruction = Instruction::decode(instruction_word);
    println!("Decoded instruction: {}", instruction);
}
