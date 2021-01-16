use crate::{function::FunctionState, value::LemonVMValue};

#[derive(Debug, Clone, Copy)]
pub enum RegType {
    // Common Registers
    // Imm value
    IMM,
    // Global variable register for current stack, Read and Write
    R,
    // Block position register, Read and Write
    RB,

    // Calling Registers
    // Caller
    // Function call return, sharing stack with callee, Read only
    RFRETS,
    // Function call args, Write only
    RFARGS,
    // Callee
    // Position to return, sharing stack with caller, Write only
    RCRETS,
    // Position to get args, Read only
    RCARGS,
}
#[derive(Debug, Clone, Copy)]
pub struct Reg {
    pub ty: RegType,
    pub value: u64,
}

impl Reg {
    pub fn read(&self, state: &FunctionState) -> LemonVMValue {
        match self.ty {
            RegType::IMM => LemonVMValue::new_u64(self.value),
            RegType::R => state.stack.r_ptr[self.value as usize],
            RegType::RB => {
                let block_bot = *state.blocks_index.last().expect("FUCK NO BLOCK");
                state.stack.r_ptr[block_bot as usize + self.value as usize]
            }
            RegType::RFRETS => {
                if let Some(func_call) = &state.as_caller {
                    func_call.rets_ptr[self.value as usize]
                } else {
                    panic!("ACCESSING FUNCTION CALL REGISTERS WITHOUT PUSHING FRAME")
                }
            }
            RegType::RFARGS => panic!("WRITE ONLY REGISTER"),
            RegType::RCRETS => panic!("WRITE ONLY REGISTER"),
            RegType::RCARGS => state.stack.call_header.args_ptr[self.value as usize],
        }
    }
    pub fn write(&self, value: LemonVMValue, state: &mut FunctionState) {
        match self.ty {
            RegType::IMM => panic!("WRITTING TO AN IMM REGISTER????"),
            RegType::R => {
                state.stack.r_ptr[self.value as usize] = value;
            }
            RegType::RB => {
                let block_bot = *state.blocks_index.last().expect("FUCK NO BLOCK");
                state.stack.r_ptr[block_bot as usize + self.value as usize] = value;
            }
            RegType::RFRETS => panic!("WRTTING TO READ ONLY REGISTER"),
            RegType::RFARGS => {
                state.stack.call_header.args_ptr[self.value as usize] = value;
            }
            RegType::RCRETS => {
                state.stack.call_header.rets_ptr[self.value as usize] = value;
            }
            RegType::RCARGS => panic!("WRTTING TO READ ONLY REGISTER"),
        }
    }
}
