//! Storage Binary Search Tree, is a binary search tree or BST that instead of using Random Access Memory,
//! Read and writes from contract's storage. Therefore it's permanently store inside contract's storage.

use super::error::Error;
use crate::collections::bst::header::Header;
use crate::collections::bst::node::Node;
use crate::error::HostError;
use crate::storage::{sread_struct, swrite_struct, Storage};
use core::marker::PhantomData;
use core::mem::size_of;
use core::result::Result;

/// The instance of Storage Binary Search Tree
pub struct StorageBST<'a, K, V>
where
    K: Sized + Ord,
    V: Sized,
{
    storage: &'a dyn Storage,
    offset: u32,
    header: Header,
    _phantom: PhantomData<(K, V)>,
}

impl<'a, K, V> StorageBST<'a, K, V>
where
    K: Sized + Ord,
    V: Sized,
{
    /// creates ans store a new instance of Storage Binary Search Tree at the given offset
    pub fn create(storage: &'a dyn Storage, offset: u32) -> Result<Self, Error> {
        let header = Header::new::<K, V>();
        swrite_struct(storage, offset, &header)?;

        Ok(StorageBST {
            storage,
            offset,
            header,
            _phantom: PhantomData,
        })
    }
    /// load the Storage Binary Search Tree
    pub fn lazy_load(storage: &'a dyn Storage, offset: u32) -> Result<Self, Error> {
        let header = sread_struct::<Header>(storage, offset)?;

        // TODO:
        // Check boom and reserved field to be correct

        if header.key_size != size_of::<K>() as u16 {
            return Err(Error::InvalidOffset(offset));
        }

        if header.value_size != size_of::<V>() as u16 {
            return Err(Error::InvalidOffset(offset));
        }

        Ok(StorageBST {
            storage,
            offset,
            header,
            _phantom: PhantomData,
        })
    }

    /// Inserts a key-value pair into the tree.
    /// If the map did not have this key present, None is returned.
    /// If the map did have this key present, the value is updated, and the old value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, HostError> {
        if self.header.count == 0 {
            // create a root node
            let root = Node::new(key, value);
            self.header.count = 1;

            let root_offset = self.offset + size_of::<Header>() as u32;
            swrite_struct(self.storage, self.offset, &self.header)?;
            swrite_struct(self.storage, root_offset, &root)?;
            Ok(None)
        } else {
            let mut offset = self.offset + size_of::<Header>() as u32;
            let mut node = sread_struct::<Node<K, V>>(self.storage, offset)?;

            loop {
                if node.key.eq(&key) {
                    let old_value = node.value;
                    node.value = value;
                    swrite_struct(self.storage, offset, &node)?;
                    return Ok(Some(old_value));
                } else if node.key.le(&key) {
                    if node.left.eq(&0) {
                        self.header.count += 1;
                        let new_offset = self.offset
                            + size_of::<Header>() as u32
                            + (self.header.count * size_of::<Node<K, V>>() as u32);

                        swrite_struct(self.storage, self.offset, &self.header)?;
                        node.left = new_offset;
                        swrite_struct(self.storage, offset, &node)?;
                        let new_node = Node::new(key, value);
                        swrite_struct(self.storage, new_offset, &new_node)?;
                        return Ok(None);
                    }
                    offset = node.left;
                } else {
                    if node.right.eq(&0) {
                        self.header.count += 1;
                        let new_offset = self.offset
                            + size_of::<Header>() as u32
                            + (self.header.count * size_of::<Node<K, V>>() as u32);

                        swrite_struct(self.storage, self.offset, &self.header)?;
                        node.right = new_offset;
                        swrite_struct(self.storage, offset, &node)?;
                        let new_node = Node::new(key, value);
                        swrite_struct(self.storage, new_offset, &new_node)?;
                        return Ok(None);
                    }
                    offset = node.right;
                }
                node = sread_struct::<Node<K, V>>(self.storage, offset)?;
            }
        }
    }

    /// Returns the value corresponding to the key. If the key doesn't exists, it returns None.
    pub fn find(&self, key: &K) -> Result<Option<V>, HostError> {
        if self.header.count == 0 {
            return Ok(None);
        }

        let mut offset = self.offset + size_of::<Header>() as u32;
        let mut node = sread_struct::<Node<K, V>>(self.storage, offset)?;

        loop {
            if node.key.eq(key) {
                return Ok(Some(node.value));
            } else if node.key.le(key) {
                if node.left.eq(&0) {
                    return Ok(None);
                }
                offset = node.left;
            } else {
                if node.left.eq(&0) {
                    return Ok(None);
                }
                offset = node.right;
            }
            node = sread_struct::<Node<K, V>>(self.storage, offset)?;
        }
    }

    /// Returns true if the tree contains a value for the specified key.
    pub fn contains_key(&self, key: &K) -> Result<bool, HostError> {
        Ok(self.find(key)?.is_some())
    }
}

#[cfg(test)]
mod tests {
    use crate::mock::mock_storage;

    use super::*;
    use core::mem::size_of;

    #[test]
    fn test_size() {
        assert_eq!(16, size_of::<Header>());
        assert_eq!(24, size_of::<Node<i64, i64>>());
        assert_eq!(12, size_of::<Node<i16, i16>>());
        assert_eq!(12, size_of::<Node<i8, i16>>());
        assert_eq!(16, size_of::<Node<i8, i32>>());
    }

    #[test]
    fn test_header() {
        let storage = mock_storage(1024);
        StorageBST::<i32, i32>::create(&storage, 512).unwrap();
        let header = sread_struct::<Header>(&storage, 512).unwrap();
        assert_eq!(header.boom, 0xb3000000);
        assert_eq!(header.key_size, 4);
        assert_eq!(header.value_size, 4);
        assert_eq!(header.count, 0);
        assert_eq!(header.reserved, 0);
    }

    #[test]
    fn test_bst() {
        let storage = mock_storage(1024);
        let mut bst = StorageBST::<i32, i32>::create(&storage, 512).unwrap();
        assert_eq!(None, bst.find(&0).unwrap());
        bst.insert(0, 0).unwrap();
        assert_eq!(Some(0), bst.find(&0).unwrap());

        assert_eq!(None, bst.insert(3, 30).unwrap());
        assert_eq!(None, bst.insert(2, 20).unwrap());
        assert_eq!(None, bst.insert(1, 10).unwrap());
        assert_eq!(None, bst.insert(4, 40).unwrap());
        assert_eq!(Some(0), bst.insert(0, 100).unwrap());

        assert_eq!(Some(30), bst.find(&3).unwrap());
        assert_eq!(Some(100), bst.find(&0).unwrap());
        assert!(bst.contains_key(&2).unwrap());
        assert!(!bst.contains_key(&8).unwrap());
    }
}
