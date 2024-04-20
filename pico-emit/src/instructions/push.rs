use crate::impl_opcode;
use crate::registers::types::RegisterList;

pub trait Push<List> {
    fn push(&mut self, list: List);
}

impl_opcode!(Push, push, (8, 0b1011_0_100), (0, list: RegisterList<false, false>));
impl_opcode!(Push, push, (8, 0b1011_0_101), (0, list: RegisterList<true, false>));
