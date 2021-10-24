//! The context for running contract actor

use crate::error::KelkError;
use crate::params::*;
use alloc::vec::Vec;
use core::cell::RefCell;

/// TODO
pub trait ContextAPI {
    /// TODO
    fn msg_sender(&self) -> Result<Vec<u8>, KelkError>;

    /// TODO
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError>;

    /// TODO
    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError>;

    /// TODO
    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError>;
}

/// TODO
pub struct OwnedContext<C: ContextAPI> {
    /// TODO
    pub api: C,
}

/// TODO
pub struct ContextMut<'a> {
    /// TODO
    pub api: &'a dyn ContextAPI,
}

/// TODO
#[derive(Copy, Clone)]
pub struct Context<'a> {
    /// TODO
    pub api: &'a dyn ContextAPI,
}

/// TODO
impl<C: ContextAPI> OwnedContext<C> {
    /// TODO
    pub fn as_ref(&'_ self) -> Context<'_> {
        Context { api: &self.api }
    }

    /// TODO
    pub fn as_mut(&'_ mut self) -> ContextMut<'_> {
        ContextMut { api: &self.api }
    }
}

/// TODO
pub struct ContextExt {}

impl ContextExt {
    /// TODO
    pub fn new() -> Self {
        ContextExt {}
    }
}

impl ContextAPI for ContextExt {
    fn msg_sender(&self) -> Result<Vec<u8>, KelkError> {
        unimplemented!();
    }
    /// TODO
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        unimplemented!();
    }

    /// TODO
    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError> {
        unimplemented!();
    }

    /// TODO
    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!();
    }
}

///todo
pub struct MockAPI {
    storage: RefCell<[u8; u16::MAX as usize]>,
}

impl MockAPI {
    ///todo
    pub fn new() -> Self {
        // ContextMut{
        //     api:MockContext{
        //         storage: RefCell::new([0u8;512]),
        //     }
        // }
        MockAPI {
            storage: RefCell::new([0u8; u16::MAX as usize]),
        }
    }
}

impl ContextAPI for MockAPI {
    fn write_storage(&self, offset: u32, data: &[u8]) -> Result<(), KelkError> {
        for i in 0..data.len() - 1 {
            let mut store = self.storage.borrow_mut();
            store[offset as usize + i] = data[i];
        }
        Ok(())
    }

    fn read_storage(&self, offset: u32, length: u32) -> Result<Vec<u8>, KelkError> {
        let c = &self.storage.borrow()[offset as usize..offset as usize + length as usize];
        Ok(c.into())
    }

    fn msg_sender(&self) -> Result<Vec<u8>, KelkError> {
        let c = b"zrb1rwchw6xq0fqj45c9p3lagx9tgu7l9jgzjglp0v";
        Ok(c.to_vec())
    }

    fn get_param(&self, param_id: i32) -> Result<ParamType, KelkError> {
        unimplemented!()
    }
}
