pub mod bl602;
pub mod bl616;
pub use crate::elf::{CodeSegment, FirmwareImage, RomSegment};
use crate::image::{BootHeaderCfg, PartitionCfg};
use crate::Error;
pub use bl602::Bl602;
pub use bl616::Bl616;
use std::str::FromStr;

pub trait Chip {
    fn target(&self) -> &'static str;
    fn get_eflash_loader(&self) -> &[u8];
    fn get_flash_segment<'a>(&self, code_segment: CodeSegment<'a>) -> Option<RomSegment<'a>>;
    fn with_boot2(
        &self,
        partition_cfg: PartitionCfg,
        bootheader_cfg: BootHeaderCfg,
        ro_params: Vec<u8>,
        bin: &[u8],
    ) -> Result<Vec<RomSegment>, Error>;
}

pub enum ChipName {
    BL602(Bl602),
    BL616(Bl616),
}

// Implement parsing for ChipName
impl FromStr for ChipName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BL602" => Ok(ChipName::BL602(Bl602)),
            "BL616" => Ok(ChipName::BL616(Bl616)),
            _ => Ok(ChipName::BL602(Bl602)),
            // use bl602 as default, no errors
            // _ => Err(Error),
        }
    }
}
