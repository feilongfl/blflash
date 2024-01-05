use num_enum::TryFromPrimitive;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IO error while using serial port: {0}")]
    Serial(#[from] serial::core::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to connect to the device")]
    ConnectionFailed,
    #[error("Timeout while running command")]
    Timeout,
    #[error("Invalid response header")]
    RespError,
    #[error("Packet to large for buffer")]
    OverSizedPacket,
    #[error("elf image is not valid")]
    InvalidElf,
    #[error("elf image can not be ran from ram")]
    ElfNotRamLoadable,
    #[error("chip not recognized")]
    UnrecognizedChip,
    #[error("flash chip not supported, flash id: {0:#x}")]
    UnsupportedFlash(u8),
    #[error("ROM error {0:?}")]
    RomError(RomError),
    #[error("Parse error")]
    ParseError(#[from] deku::error::DekuError),
    #[error("Parse toml error")]
    TomlError(#[from] toml::de::Error),
}

#[derive(Copy, Clone, Debug, TryFromPrimitive)]
#[allow(dead_code)]
#[repr(u16)]
pub enum RomError {
    Success = 0x0000,
    FlashInitError = 0x0001,
    FlashEraseParaError = 0x0002,
    FlashEraseError = 0x0003,
    FlashWriteParaError = 0x0004,
    FlashWriteAddrError = 0x0005,
    FlashWriteError = 0x0006,
    FlashBootPara = 0x0007,
    CmdIdError = 0x0101,
    CmdLenError = 0x0102,
    CmdCrcError = 0x0103,
    CmdSeqError = 0x0104,
    ImgBootheaderLenError = 0x0201,
    ImgBootheaderNotLoadError = 0x0202,
    ImgBootheaderMagicError = 0x0203,
    ImgBootheaderCrcError = 0x0204,
    ImgBootheaderEncryptNotfit = 0x0205,
    ImgBootheaderSignNotfit = 0x0206,
    ImgSegmentCntError = 0x0207,
    ImgAesIvLenError = 0x0208,
    ImgAesIvCrcError = 0x0209,
    ImgPkLenError = 0x020a,
    ImgPkCrcError = 0x020b,
    ImgPkHashError = 0x020c,
    ImgSignatureLenError = 0x020d,
    ImgSignatureCrcError = 0x020e,
    ImgSectionheaderLenError = 0x020f,
    ImgSectionheaderCrcError = 0x0210,
    ImgSectionheaderDstError = 0x0211,
    ImgSectiondataLenError = 0x0212,
    ImgSectiondataDecError = 0x0213,
    ImgSectiondataTlenError = 0x0214,
    ImgSectiondataCrcError = 0x0215,
    ImgHalfbakedError = 0x0216,
    ImgHashError = 0x0217,
    ImgSignParseError = 0x0218,
    ImgSignError = 0x0219,
    ImgDecError = 0x021a,
    ImgAllInvalidError = 0x021b,
    IfRateLenError = 0x0301,
    IfRateParaError = 0x0302,
    IfPasswordError = 0x0303,
    IfPasswordClose = 0x0304,
    PllError = 0xfffc,
    InvasionError = 0xfffd,
    Polling = 0xfffe,
    Fail = 0xffff,

    Unknow = 0x8fff,
}
