//! Kelk TODO

use core::cell::RefCell;

use alloc::vec::Vec;

use crate::{context::ContextAPI, error::KelkError, params::ParamType};

const MOCK_SIZE: usize = 65535;

///todo
pub struct MockContext {
    storage: RefCell<[u8; u16::MAX as usize]>,
}

///todo
impl MockContext {
    ///todo
    pub fn new() -> Self {
        MockContext {
            storage: RefCell::new([0u8; MOCK_SIZE]),
        }
    }
}

impl ContextAPI for MockContext {
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
