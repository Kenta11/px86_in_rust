use strum_macros::EnumIter;
use variant_count::VariantCount;

#[derive(Clone, Copy, Debug, EnumIter, VariantCount)]
pub enum Register32 {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

#[derive(Clone, Copy, Debug, EnumIter, VariantCount)]
pub enum Register8 {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
}

#[derive(Debug)]
pub struct Emulator {
    pub registers: [u32; Register32::VARIANT_COUNT],
    pub eflags: u16,
    pub memory: Vec<u8>,
    pub eip: u32,
}
