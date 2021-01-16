use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct GCHeader {
    id: usize,
    read_barrier: usize,
    write_barrier: usize,
}

#[derive(Copy, Clone)]
pub union LemonVMInnerValue {
    // value type
    bool: bool,
    u64: u64,
    f64: f64,
    // fix lengthed array on stack
    // in_stack_array: StackPtr<LemonVMInnerValue>,
    // box type
    ref_type: GCHeader,
}

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LemonVMValueType {
    U64 = 0,
    F64 = 1,
    BOOL = 2,
}

#[derive(Copy, Clone)]
pub struct LemonVMValue {
    ty: LemonVMValueType,
    value: LemonVMInnerValue,
}

impl Debug for LemonVMValue {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl LemonVMValue {
    #[inline(always)]
    pub fn add(&self, other: &Self) -> Self {
        // TODO: check
        assert!(self.ty == other.ty && self.ty == LemonVMValueType::U64);
        unsafe { Self::new_u64(self.value.u64 + other.value.u64) }
    }
    #[inline(always)]
    pub fn sub(&self, other: &Self) -> Self {
        // TODO: check
        assert!(self.ty == other.ty && self.ty == LemonVMValueType::U64);
        unsafe { Self::new_u64(self.value.u64 - other.value.u64) }
    }
    #[inline(always)]
    pub fn le(&self, other: &Self) -> Self {
        // TODO: check
        assert!(self.ty == other.ty && self.ty == LemonVMValueType::U64);
        unsafe { Self::new_bool(self.value.u64 < other.value.u64) }
    }
    #[inline(always)]
    pub fn new_u64(value: u64) -> Self {
        Self {
            ty: LemonVMValueType::U64,
            value: LemonVMInnerValue { u64: value },
        }
    }
    #[inline(always)]
    pub fn u64(&self) -> u64 {
        // TODO: check
        unsafe { self.value.u64 }
    }
    #[inline(always)]
    pub fn bool(&self) -> bool {
        // TODO: check
        unsafe { self.value.bool }
    }
    #[inline(always)]
    pub fn new_bool(value: bool) -> Self {
        Self {
            ty: LemonVMValueType::BOOL,
            value: LemonVMInnerValue { bool: value },
        }
    }
    #[inline(always)]
    pub fn read(&self) {}
    #[inline(always)]
    pub fn write(&self) {}
}
