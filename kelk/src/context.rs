//! The context for running contract actor

#![feature(alloc)]
extern crate alloc;


use core::{borrow::BorrowMut, cell::RefCell, ops::IndexMut};

use crate::error::KelkError;
use alloc::{boxed::Box, vec::Vec};

/// TODO
pub trait ContextAPI {
    /// TODO
    fn write_storage(&self, offset: usize, data: &[u8]) -> Result<(), KelkError>;

    /// TODO
    fn read_storage(&self, offset: usize, length: usize) -> Result<Vec<u8>, KelkError>;
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
    fn write_storage(&self, offset: usize, data: &[u8]) -> Result<(), KelkError> {
        todo!("unimplemented");
    }

    fn read_storage(&self, offset: usize, length: usize) -> Result<Vec<u8>, KelkError> {
        todo!("unimplemented");
    }
}


///todo
pub struct MockContext {
    storage: RefCell<[u8;512]>,
}

impl MockContext {
    ///todo
    pub fn new() -> Self {
        MockContext {
            storage: RefCell::new([0u8;512]),
        }
    }
}

impl ContextAPI for MockContext {
    fn write_storage(&self, offset: usize, data: &[u8]) -> Result<(), KelkError> {
        for i in 0..data.len() - 1 {
            let mut store=self.storage.borrow_mut();
            store[offset+i]=data[i];
        }
        Ok(())
    }

    fn read_storage(&self, offset: usize, length: usize) -> Result<Vec<u8>, KelkError> {
        let c = &self.storage.borrow()[offset..length];
        Ok(c.into())
    }
}
