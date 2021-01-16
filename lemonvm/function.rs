use crate::{
    instruction::LemonVMInstruction, state::GlobalState, utils::StackPtr, value::LemonVMValue,
};
use smallvec::SmallVec;
/* INIT: constant pool */
#[derive(Debug, Clone, Copy)]
pub struct FunctionHeader {
    pub name: &'static str,
    pub args_count: u8,
    pub rets_count: u8,
    pub max_stack_usage: usize,
    pub instructions: &'static [LemonVMInstruction],
}
#[derive(Debug, Copy, Clone)]
pub struct FunctionCallHeader {
    pub args_ptr: StackPtr<LemonVMValue>,
    pub rets_ptr: StackPtr<LemonVMValue>,
}

#[derive(Debug, Copy, Clone)]
pub struct FunctionStackPtr {
    pub call_header: FunctionCallHeader,
    pub r_ptr: StackPtr<LemonVMValue>,
}

/* INIT: enter global state */
#[derive(Debug)]
pub struct FunctionState {
    /* INIT: constant pool */
    pub header: FunctionHeader,
    // pub stack_global_location: usize,

    // size from header.max_stack_usage
    pub stack: FunctionStackPtr,

    pub as_caller: Option<FunctionCallHeader>,
    pub blocks_index: SmallVec<[u16; 64]>,
    pub pc: u32,
    // pub on_error: Option<Error>,
}
impl FunctionState {
    pub fn run(&mut self, global_state: &mut GlobalState) {
        let ins = self.header.instructions;
        while self.pc < ins.len() as u32 {
            ins[self.pc as usize].on_fetch(self, global_state);
        }
    }
}

pub struct Error {
    domain: usize,             // string, where cause the error
    error_code: usize,         // type of that error
    state: *mut FunctionState, // meta info for function
}

// Closure in LemonVM Kit follows flat closure
// flat closure:
//  a = 0
//  def f():
//  return
//      def g():
//          return ++a
// a in f  = &a
// a in g = a in f
pub struct Closure {
    /* INIT: constant pool */
    pub header: FunctionHeader,

    pub capture_table: usize,
}
