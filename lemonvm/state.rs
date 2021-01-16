use crate::{function::*, utils::StackPtr, value::LemonVMValue};
use smallvec::SmallVec;

use core::{fmt::Debug, intrinsics::likely};

// [final rets]
// ⬆️ [args1 func1 ret2 |⬅️ args2 func2 ]
#[derive(Debug)]
pub struct GlobalState {
    pub final_return: StackPtr<LemonVMValue>,
    pub global_stack: StackPtr<LemonVMValue>,
    pub function_call_stack: SmallVec<[FunctionState; 512]>,
}
impl GlobalState {
    pub fn new_with_main(
        stack_ptr: *mut LemonVMValue,
        stack_size: usize,
        rets_ptr: *mut LemonVMValue,
        header: &FunctionHeader,
    ) -> Self {
        let mut global_stack = StackPtr::from_ptr(stack_ptr, stack_size);
        let ret_space = StackPtr::from_ptr(rets_ptr, header.rets_count as usize);
        let args_space = global_stack.reserve_space_from_current_stack(header.args_count as usize);
        let function_space = global_stack.reserve_space_from_current_stack(header.max_stack_usage);
        let function_call_header = FunctionCallHeader {
            args_ptr: args_space,
            rets_ptr: ret_space,
        };
        let function_stack = FunctionStackPtr {
            call_header: function_call_header,
            r_ptr: function_space,
        };
        let function_call_stack = smallvec::smallvec![FunctionState {
            header: *header,
            stack: function_stack,
            as_caller: None,
            blocks_index: SmallVec::new(),
            pc: 0
        }];
        Self {
            final_return: ret_space,
            global_stack,
            function_call_stack,
        }
    }

    pub fn push_frame(&mut self, header: &FunctionHeader) {
        let new_function_state: FunctionState;

        if let Some(func) = self.function_call_stack.last_mut() {
            // not main

            // caller sharing stack with callee
            let ret_space = func
                .stack
                .r_ptr
                .reserve_space_from_current_stack(header.rets_count as usize);
            let args_space = self
                .global_stack
                .reserve_space_from_current_stack(header.args_count as usize);
            let function_space = self
                .global_stack
                .reserve_space_from_current_stack(header.max_stack_usage);
            let function_call_header = FunctionCallHeader {
                args_ptr: args_space,
                rets_ptr: ret_space,
            };
            let function_stack = FunctionStackPtr {
                call_header: function_call_header,
                r_ptr: function_space,
            };
            new_function_state = FunctionState {
                header: *header,
                stack: function_stack,
                as_caller: None,
                blocks_index: SmallVec::new(),
                pc: 0,
            };
            func.as_caller = Some(function_call_header);
        } else {
            // main but not here to handle

            // directly return to final_return
            let ret_space = if likely(self.final_return.size > header.rets_count as usize) {
                self.final_return
            } else {
                panic!("no enought space for main to return");
            };
            let args_space = self
                .global_stack
                .reserve_space_from_current_stack(header.args_count as usize);
            let function_space = self
                .global_stack
                .reserve_space_from_current_stack(header.max_stack_usage);
            let function_call_header = FunctionCallHeader {
                args_ptr: args_space,
                rets_ptr: ret_space,
            };
            let function_stack = FunctionStackPtr {
                call_header: function_call_header,
                r_ptr: function_space,
            };
            new_function_state = FunctionState {
                header: *header,
                stack: function_stack,
                as_caller: None,
                blocks_index: SmallVec::new(),
                pc: 0,
            };
        }

        self.function_call_stack.push(new_function_state);
    }
    pub fn pop_frame_continue(&mut self) {
        if let Some(func) = self.function_call_stack.pop() {
            self.global_stack
                .remove_space_from_current_stack(func.header.max_stack_usage);
            self.global_stack
                .remove_space_from_current_stack(func.header.args_count as usize);
            self.function_call_stack
                .last_mut()
                .map(|f| f.as_caller = None);
            self.run();
        } else {
            panic!("FINISH!");
        }
    }
    pub fn run(&mut self) {
        // TODO: fuck me!
        let fuck_self = unsafe { &mut *(self as *mut GlobalState) };
        self.function_call_stack
            .last_mut()
            .map(|f| f.run(fuck_self));
    }

    pub fn get_header(&self, _id: u32) -> FunctionHeader {
        todo!()
    }
}
