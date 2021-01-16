#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum InstructionSet {
    // X
    NOP,
    // AB
    // IMMNUM,
    // AB
    MOV,
    // ABC
    ADD,
    // ABC
    SUB,
    // ABC
    LE,
    // ABC
    GT,
    // ABC
    EQ,

    // ABC
    AND,
    // ABC
    OR,
    // AB
    NOT,

    // ABC
    IF,

    // ALL
    PUSHF,
    // AB
    // PUSHA,
    // ALL
    CALL,
    // X
    RET,
    // ALL
    // Push a block to set RB register position
    PUSHB,
    // X
    // Pops a block to reset RB register position
    POPB,
}
