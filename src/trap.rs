#[derive(Debug)]
pub enum Trap {
    IllegalInstruction,
}

#[derive(Debug)]
pub enum TrapRetInstruction {
    Sret,
    Mret,
}