use crate::block::BlockType::Air;

pub struct Block{
    pub block_type: BlockType

}

#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum BlockType{
    Air,
    Grass,
    Stone
}

impl BlockType {
    pub(crate) fn idx(&self) -> usize { *self as usize }

    pub fn is_solid(&self) -> bool{ *self != Air }
}