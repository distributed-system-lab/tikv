// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::{
    fmt::{self, Debug, Formatter},
    ops::Deref,
};

use engine_traits::DBVector;
use rocksdb::DBVector as RawDBVector;

pub struct RocksDBVector(RawDBVector);

impl RocksDBVector {
    pub fn from_raw(raw: RawDBVector) -> RocksDBVector {
        RocksDBVector(raw)
    }
}

impl DBVector for RocksDBVector {}

impl Deref for RocksDBVector {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for RocksDBVector {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", &**self)
    }
}

impl<'a> PartialEq<&'a [u8]> for RocksDBVector {
    fn eq(&self, rhs: &&[u8]) -> bool {
        **rhs == **self
    }
}
