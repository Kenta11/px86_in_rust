use strum_macros::EnumIter;
use variant_count::VariantCount;

#[derive(Clone, Copy, Debug, EnumIter, VariantCount)]
pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

#[derive(Debug)]
pub struct Emulator {
    pub registers: [u32; Register::VARIANT_COUNT],
    pub memory: Vec<u8>,
    pub eip: u32,
}
