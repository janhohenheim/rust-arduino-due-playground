#[doc = "Register `CIDR` reader"]
pub type R = crate::R<CIDR_SPEC>;
#[doc = "Field `VERSION` reader - Version of the Device"]
pub type VERSION_R = crate::FieldReader;
#[doc = "Field `EPROC` reader - Embedded Processor"]
pub type EPROC_R = crate::FieldReader<EPROC_A>;
#[doc = "Embedded Processor"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPROC_A {
    #[doc = "1: ARM946ES"]
    ARM946ES = 1,
    #[doc = "2: ARM7TDMI"]
    ARM7TDMI = 2,
    #[doc = "3: Cortex-M3"]
    CM3 = 3,
    #[doc = "4: ARM920T"]
    ARM920T = 4,
    #[doc = "5: ARM926EJS"]
    ARM926EJS = 5,
    #[doc = "6: Cortex-A5"]
    CA5 = 6,
    #[doc = "7: Cortex-M4"]
    CM4 = 7,
}
impl From<EPROC_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPROC_A {
    type Ux = u8;
}
impl EPROC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPROC_A> {
        match self.bits {
            1 => Some(EPROC_A::ARM946ES),
            2 => Some(EPROC_A::ARM7TDMI),
            3 => Some(EPROC_A::CM3),
            4 => Some(EPROC_A::ARM920T),
            5 => Some(EPROC_A::ARM926EJS),
            6 => Some(EPROC_A::CA5),
            7 => Some(EPROC_A::CM4),
            _ => None,
        }
    }
    #[doc = "ARM946ES"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROC_A::ARM946ES
    }
    #[doc = "ARM7TDMI"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROC_A::ARM7TDMI
    }
    #[doc = "Cortex-M3"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == EPROC_A::CM3
    }
    #[doc = "ARM920T"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROC_A::ARM920T
    }
    #[doc = "ARM926EJS"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROC_A::ARM926EJS
    }
    #[doc = "Cortex-A5"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == EPROC_A::CA5
    }
    #[doc = "Cortex-M4"]
    #[inline(always)]
    pub fn is_cm4(&self) -> bool {
        *self == EPROC_A::CM4
    }
}
#[doc = "Field `NVPSIZ` reader - Nonvolatile Program Memory Size"]
pub type NVPSIZ_R = crate::FieldReader<NVPSIZ_A>;
#[doc = "Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPSIZ_A {
    type Ux = u8;
}
impl NVPSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ_A> {
        match self.bits {
            0 => Some(NVPSIZ_A::NONE),
            1 => Some(NVPSIZ_A::_8K),
            2 => Some(NVPSIZ_A::_16K),
            3 => Some(NVPSIZ_A::_32K),
            5 => Some(NVPSIZ_A::_64K),
            7 => Some(NVPSIZ_A::_128K),
            9 => Some(NVPSIZ_A::_256K),
            10 => Some(NVPSIZ_A::_512K),
            12 => Some(NVPSIZ_A::_1024K),
            14 => Some(NVPSIZ_A::_2048K),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ_A::NONE
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ_A::_8K
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ_A::_16K
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ_A::_32K
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ_A::_64K
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ_A::_128K
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ_A::_256K
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ_A::_512K
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ_A::_1024K
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ_A::_2048K
    }
}
#[doc = "Field `NVPSIZ2` reader - Second Nonvolatile Program Memory Size"]
pub type NVPSIZ2_R = crate::FieldReader<NVPSIZ2_A>;
#[doc = "Second Nonvolatile Program Memory Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPSIZ2_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: 8 Kbytes"]
    _8K = 1,
    #[doc = "2: 16 Kbytes"]
    _16K = 2,
    #[doc = "3: 32 Kbytes"]
    _32K = 3,
    #[doc = "5: 64 Kbytes"]
    _64K = 5,
    #[doc = "7: 128 Kbytes"]
    _128K = 7,
    #[doc = "9: 256 Kbytes"]
    _256K = 9,
    #[doc = "10: 512 Kbytes"]
    _512K = 10,
    #[doc = "12: 1024 Kbytes"]
    _1024K = 12,
    #[doc = "14: 2048 Kbytes"]
    _2048K = 14,
}
impl From<NVPSIZ2_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPSIZ2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPSIZ2_A {
    type Ux = u8;
}
impl NVPSIZ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPSIZ2_A> {
        match self.bits {
            0 => Some(NVPSIZ2_A::NONE),
            1 => Some(NVPSIZ2_A::_8K),
            2 => Some(NVPSIZ2_A::_16K),
            3 => Some(NVPSIZ2_A::_32K),
            5 => Some(NVPSIZ2_A::_64K),
            7 => Some(NVPSIZ2_A::_128K),
            9 => Some(NVPSIZ2_A::_256K),
            10 => Some(NVPSIZ2_A::_512K),
            12 => Some(NVPSIZ2_A::_1024K),
            14 => Some(NVPSIZ2_A::_2048K),
            _ => None,
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2_A::NONE
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2_A::_8K
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2_A::_16K
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2_A::_32K
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2_A::_64K
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2_A::_128K
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2_A::_256K
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2_A::_512K
    }
    #[doc = "1024 Kbytes"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2_A::_1024K
    }
    #[doc = "2048 Kbytes"]
    #[inline(always)]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2_A::_2048K
    }
}
#[doc = "Field `SRAMSIZ` reader - Internal SRAM Size"]
pub type SRAMSIZ_R = crate::FieldReader<SRAMSIZ_A>;
#[doc = "Internal SRAM Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRAMSIZ_A {
    #[doc = "0: 48 Kbytes"]
    _48K = 0,
    #[doc = "1: 192 Kbytes"]
    _192K = 1,
    #[doc = "2: 2 Kbytes"]
    _2K = 2,
    #[doc = "3: 6 Kbytes"]
    _6K = 3,
    #[doc = "4: 24 Kbytes"]
    _24K = 4,
    #[doc = "5: 4 Kbytes"]
    _4K = 5,
    #[doc = "6: 80 Kbytes"]
    _80K = 6,
    #[doc = "7: 160 Kbytes"]
    _160K = 7,
    #[doc = "8: 8 Kbytes"]
    _8K = 8,
    #[doc = "9: 16 Kbytes"]
    _16K = 9,
    #[doc = "10: 32 Kbytes"]
    _32K = 10,
    #[doc = "11: 64 Kbytes"]
    _64K = 11,
    #[doc = "12: 128 Kbytes"]
    _128K = 12,
    #[doc = "13: 256 Kbytes"]
    _256K = 13,
    #[doc = "14: 96 Kbytes"]
    _96K = 14,
    #[doc = "15: 512 Kbytes"]
    _512K = 15,
}
impl From<SRAMSIZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SRAMSIZ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRAMSIZ_A {
    type Ux = u8;
}
impl SRAMSIZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMSIZ_A {
        match self.bits {
            0 => SRAMSIZ_A::_48K,
            1 => SRAMSIZ_A::_192K,
            2 => SRAMSIZ_A::_2K,
            3 => SRAMSIZ_A::_6K,
            4 => SRAMSIZ_A::_24K,
            5 => SRAMSIZ_A::_4K,
            6 => SRAMSIZ_A::_80K,
            7 => SRAMSIZ_A::_160K,
            8 => SRAMSIZ_A::_8K,
            9 => SRAMSIZ_A::_16K,
            10 => SRAMSIZ_A::_32K,
            11 => SRAMSIZ_A::_64K,
            12 => SRAMSIZ_A::_128K,
            13 => SRAMSIZ_A::_256K,
            14 => SRAMSIZ_A::_96K,
            15 => SRAMSIZ_A::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "48 Kbytes"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZ_A::_48K
    }
    #[doc = "192 Kbytes"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZ_A::_192K
    }
    #[doc = "2 Kbytes"]
    #[inline(always)]
    pub fn is_2k(&self) -> bool {
        *self == SRAMSIZ_A::_2K
    }
    #[doc = "6 Kbytes"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZ_A::_6K
    }
    #[doc = "24 Kbytes"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZ_A::_24K
    }
    #[doc = "4 Kbytes"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZ_A::_4K
    }
    #[doc = "80 Kbytes"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZ_A::_80K
    }
    #[doc = "160 Kbytes"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZ_A::_160K
    }
    #[doc = "8 Kbytes"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZ_A::_8K
    }
    #[doc = "16 Kbytes"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZ_A::_16K
    }
    #[doc = "32 Kbytes"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZ_A::_32K
    }
    #[doc = "64 Kbytes"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZ_A::_64K
    }
    #[doc = "128 Kbytes"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZ_A::_128K
    }
    #[doc = "256 Kbytes"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZ_A::_256K
    }
    #[doc = "96 Kbytes"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZ_A::_96K
    }
    #[doc = "512 Kbytes"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZ_A::_512K
    }
}
#[doc = "Field `ARCH` reader - Architecture Identifier"]
pub type ARCH_R = crate::FieldReader<ARCH_A>;
#[doc = "Architecture Identifier"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARCH_A {
    #[doc = "131: SAM3AxC (100-pin version)"]
    SAM3AX_C = 131,
    #[doc = "132: SAM3XxC (100-pin version)"]
    SAM3XX_C = 132,
    #[doc = "133: SAM3XxE (144-pin version)"]
    SAM3XX_E = 133,
    #[doc = "134: SAM3XxG (208/217-pin version)"]
    SAM3XX_G = 134,
}
impl From<ARCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ARCH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARCH_A {
    type Ux = u8;
}
impl ARCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARCH_A> {
        match self.bits {
            131 => Some(ARCH_A::SAM3AX_C),
            132 => Some(ARCH_A::SAM3XX_C),
            133 => Some(ARCH_A::SAM3XX_E),
            134 => Some(ARCH_A::SAM3XX_G),
            _ => None,
        }
    }
    #[doc = "SAM3AxC (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3ax_c(&self) -> bool {
        *self == ARCH_A::SAM3AX_C
    }
    #[doc = "SAM3XxC (100-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_c(&self) -> bool {
        *self == ARCH_A::SAM3XX_C
    }
    #[doc = "SAM3XxE (144-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_e(&self) -> bool {
        *self == ARCH_A::SAM3XX_E
    }
    #[doc = "SAM3XxG (208/217-pin version)"]
    #[inline(always)]
    pub fn is_sam3xx_g(&self) -> bool {
        *self == ARCH_A::SAM3XX_G
    }
}
#[doc = "Field `NVPTYP` reader - Nonvolatile Program Memory Type"]
pub type NVPTYP_R = crate::FieldReader<NVPTYP_A>;
#[doc = "Nonvolatile Program Memory Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVPTYP_A {
    #[doc = "0: ROM"]
    ROM = 0,
    #[doc = "1: ROMless or on-chip Flash"]
    ROMLESS = 1,
    #[doc = "2: Embedded Flash Memory"]
    FLASH = 2,
    #[doc = "3: ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    ROM_FLASH = 3,
    #[doc = "4: SRAM emulating ROM"]
    SRAM = 4,
}
impl From<NVPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: NVPTYP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NVPTYP_A {
    type Ux = u8;
}
impl NVPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVPTYP_A> {
        match self.bits {
            0 => Some(NVPTYP_A::ROM),
            1 => Some(NVPTYP_A::ROMLESS),
            2 => Some(NVPTYP_A::FLASH),
            3 => Some(NVPTYP_A::ROM_FLASH),
            4 => Some(NVPTYP_A::SRAM),
            _ => None,
        }
    }
    #[doc = "ROM"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYP_A::ROM
    }
    #[doc = "ROMless or on-chip Flash"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYP_A::ROMLESS
    }
    #[doc = "Embedded Flash Memory"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYP_A::FLASH
    }
    #[doc = "ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYP_A::ROM_FLASH
    }
    #[doc = "SRAM emulating ROM"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYP_A::SRAM
    }
}
#[doc = "Field `EXT` reader - Extension Flag"]
pub type EXT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIDR_SPEC;
impl crate::RegisterSpec for CIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr::R`](R) reader structure"]
impl crate::Readable for CIDR_SPEC {}
