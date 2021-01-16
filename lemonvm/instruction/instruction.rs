use core::fmt::Debug;
use std::intrinsics::likely;

use crate::{function::FunctionState, registers::Reg, state::GlobalState};

use super::instruction_set::InstructionSet;

#[derive(Debug, Copy, Clone)]
pub enum InstructionMode {
    EMPTY,
    A { a: Reg },
    AB { a: Reg, b: Reg },
    ABC { a: Reg, b: Reg, c: Reg },
}

#[derive(Debug, Clone, Copy)]
pub struct LemonVMInstruction {
    pub instruction: InstructionSet,
    pub mode: InstructionMode,
}
impl LemonVMInstruction {
    pub fn on_fetch(&self, function_state: &mut FunctionState, global_state: &mut GlobalState) {
        use InstructionMode::*;
        match self.instruction {
            InstructionSet::NOP => {
                // TODO: debug print all
                println!("{:?}", function_state);
            }
            InstructionSet::MOV => {
                if let AB { a, b } = self.mode {
                    a.write(b.read(function_state), function_state);
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
            }
            //TODO: add more mode
            InstructionSet::ADD => {
                if let ABC { a, b, c } = self.mode {
                    a.write(
                        b.read(function_state).add(&c.read(function_state)),
                        function_state,
                    );
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
            }
            InstructionSet::SUB => {
                if let ABC { a, b, c } = self.mode {
                    a.write(
                        b.read(function_state).sub(&c.read(function_state)),
                        function_state,
                    );
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
            }
            InstructionSet::LE => {
                if let ABC { a, b, c } = self.mode {
                    a.write(
                        b.read(function_state).le(&c.read(function_state)),
                        function_state,
                    );
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
            }
            InstructionSet::GT => todo!(),
            InstructionSet::EQ => todo!(),
            InstructionSet::AND => todo!(),
            InstructionSet::OR => todo!(),
            InstructionSet::NOT => todo!(),
            InstructionSet::IF => {
                if let ABC { a, b, c } = self.mode {
                    if likely(a.read(function_state).bool()) {
                        function_state.pc = b.value as u32;
                    }
                    {
                        function_state.pc = c.value as u32;
                    }
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
            }
            InstructionSet::PUSHF => {
                if let A { a } = self.mode {
                    global_state.push_frame(&global_state.get_header(a.value as u32));
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
                // TODO: opt next ins will be mov
                /*
                    while(next == mov){
                        pc += 1;
                        mov(args);
                    }
                */
            }
            InstructionSet::CALL => {
                global_state.run();
            }
            InstructionSet::RET => {
                global_state.pop_frame_continue();
            }
            InstructionSet::PUSHB => {
                if let A { a } = self.mode {
                    function_state.stack.r_ptr.check_reserve(a.value as usize);
                    let top = function_state.stack.r_ptr.top;
                    function_state.blocks_index.push(top as u16);
                } else {
                    panic!("WRONG INSTRUCTION MODE");
                }
            }
            InstructionSet::POPB => {
                function_state.blocks_index.pop().expect("FUCK!");
            }
        }

        function_state.pc += 1;
    }
}

#[macro_export]
macro_rules! ins {
    ($op:ident) => {
        LemonVMInstruction {
            instruction: InstructionSet::$op,
            mode: InstructionMode::EMPTY,
        }
    };
    ($op:ident,$reg1t:ident,$reg1v:expr) => {
        LemonVMInstruction {
            instruction: InstructionSet::$op,
            mode: InstructionMode::A {
                a: Reg {
                    ty: RegType::$reg1t,
                    value: $reg1v,
                },
            },
        }
    };
    ($op:ident,$reg1t:ident,$reg1v:expr,$reg2t:ident,$reg2v:expr) => {
        LemonVMInstruction {
            instruction: InstructionSet::$op,
            mode: InstructionMode::AB {
                a: Reg {
                    ty: RegType::$reg1t,
                    value: $reg1v,
                },
                b: Reg {
                    ty: RegType::$reg2t,
                    value: $reg2v,
                },
            },
        }
    };
    ($op:ident,$reg1t:ident,$reg1v:expr,$reg2t:ident,$reg2v:expr,$reg3t:ident,$reg3v:expr) => {
        LemonVMInstruction {
            instruction: InstructionSet::$op,
            mode: InstructionMode::ABC {
                a: Reg {
                    ty: RegType::$reg1t,
                    value: $reg1v,
                },
                b: Reg {
                    ty: RegType::$reg2t,
                    value: $reg2v,
                },
                c: Reg {
                    ty: RegType::$reg3t,
                    value: $reg3v,
                },
            },
        }
    };
}
