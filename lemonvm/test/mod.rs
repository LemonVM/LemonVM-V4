use crate::{
    function::FunctionHeader, ins, instruction::*, registers::*, state::GlobalState,
    value::LemonVMValue,
};

#[test]
fn bad_instruction() {
    let mut ret = [LemonVMValue::new_u64(0); 14];
    let mut stack = [LemonVMValue::new_u64(0); 1145];
    let mut gs = GlobalState::new_with_main(
        stack.as_mut_ptr(),
        1145,
        ret.as_mut_ptr(),
        &FunctionHeader {
            name: "bad_instruction",
            args_count: 2,
            rets_count: 1,
            max_stack_usage: 0,
            instructions: &[ins!(ADD, RCRETS, 0), ins!(RET)],
        },
    );
    let err = std::panic::catch_unwind(move || {
        gs.run();
    });
    assert!(!err.is_ok());
}

#[test]
fn test_add() {
    let mut ret = [LemonVMValue::new_u64(0); 14];
    let mut stack = [LemonVMValue::new_u64(0); 1145];
    let mut gs = GlobalState::new_with_main(
        stack.as_mut_ptr(),
        1145,
        ret.as_mut_ptr(),
        &FunctionHeader {
            name: "test_add",
            args_count: 2,
            rets_count: 1,
            max_stack_usage: 0,
            instructions: &[ins!(ADD, RCRETS, 0, RCARGS, 0, RCARGS, 1), ins!(RET)],
        },
    );
    stack[0] = LemonVMValue::new_u64(1);
    stack[1] = LemonVMValue::new_u64(2);
    gs.run();
    assert!(ret[0].u64() == 3);
}
