use steel::*;

use super::{name_typecase}Account;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64
}

account!({name_typecase}Account, Counter);
