#![feature(core_intrinsics)]
mod value;
use liblemonvm::{
    function::FunctionHeader, ins, instruction::*, registers::*, state::GlobalState,
    value::LemonVMValue,
};
fn main() {
    let mut ret = [LemonVMValue::new_u64(0); 14];
    let mut stack = [LemonVMValue::new_u64(0); 1145];
    let mut gs = GlobalState::new_with_main(
        stack.as_mut_ptr(),
        1145,
        ret.as_mut_ptr(),
        &FunctionHeader {
            name: "main",
            args_count: 2,
            rets_count: 1,
            max_stack_usage: 0,
            instructions: &[
                ins!(ADD, RCRETS, 0, RCARGS, 0, RCARGS, 1),
                // ins!(PUSHB,IMM,3),
                ins!(RET),
            ],
        },
    );
    stack[0] = LemonVMValue::new_u64(1);
    stack[1] = LemonVMValue::new_u64(2);
    gs.run();
    println!("{}", ret[0].u64());
}
