use crate::impl_opcode;
use crate::registers::types::RegisterList;

pub trait Pop<List> {
    fn pop(&mut self, list: List);
}

impl_opcode!(Pop, pop, (8, 0b1011_1_100), (0, list: RegisterList<false, false>));
impl_opcode!(Pop, pop, (8, 0b1011_1_101), (0, list: RegisterList<false, true>));
