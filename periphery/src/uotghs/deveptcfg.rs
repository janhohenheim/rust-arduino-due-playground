#[doc = "Register `DEVEPTCFG[%s]` reader"]
pub type R = crate::R<DEVEPTCFG_SPEC>;
#[doc = "Register `DEVEPTCFG[%s]` writer"]
pub type W = crate::W<DEVEPTCFG_SPEC>;
#[doc = "Field `ALLOC` reader - Endpoint Memory Allocate"]
pub type ALLOC_R = crate::BitReader;
#[doc = "Field `ALLOC` writer - Endpoint Memory Allocate"]
pub type ALLOC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPBK` reader - Endpoint Banks"]
pub type EPBK_R = crate::FieldReader<EPBK_A>;
#[doc = "Endpoint Banks"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPBK_A {
    #[doc = "0: Single-bank endpoint"]
    _1_BANK = 0,
    #[doc = "1: Double-bank endpoint"]
    _2_BANK = 1,
    #[doc = "2: Triple-bank endpoint"]
    _3_BANK = 2,
}
impl From<EPBK_A> for u8 {
    #[inline(always)]
    fn from(variant: EPBK_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPBK_A {
    type Ux = u8;
}
impl EPBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPBK_A> {
        match self.bits {
            0 => Some(EPBK_A::_1_BANK),
            1 => Some(EPBK_A::_2_BANK),
            2 => Some(EPBK_A::_3_BANK),
            _ => None,
        }
    }
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == EPBK_A::_1_BANK
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == EPBK_A::_2_BANK
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == EPBK_A::_3_BANK
    }
}
#[doc = "Field `EPBK` writer - Endpoint Banks"]
pub type EPBK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, EPBK_A>;
impl<'a, REG, const O: u8> EPBK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-bank endpoint"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut crate::W<REG> {
        self.variant(EPBK_A::_1_BANK)
    }
    #[doc = "Double-bank endpoint"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut crate::W<REG> {
        self.variant(EPBK_A::_2_BANK)
    }
    #[doc = "Triple-bank endpoint"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut crate::W<REG> {
        self.variant(EPBK_A::_3_BANK)
    }
}
#[doc = "Field `EPSIZE` reader - Endpoint Size"]
pub type EPSIZE_R = crate::FieldReader<EPSIZE_A>;
#[doc = "Endpoint Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPSIZE_A {
    #[doc = "0: 8 bytes"]
    _8_BYTE = 0,
    #[doc = "1: 16 bytes"]
    _16_BYTE = 1,
    #[doc = "2: 32 bytes"]
    _32_BYTE = 2,
    #[doc = "3: 64 bytes"]
    _64_BYTE = 3,
    #[doc = "4: 128 bytes"]
    _128_BYTE = 4,
    #[doc = "5: 256 bytes"]
    _256_BYTE = 5,
    #[doc = "6: 512 bytes"]
    _512_BYTE = 6,
    #[doc = "7: 1024 bytes"]
    _1024_BYTE = 7,
}
impl From<EPSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPSIZE_A {
    type Ux = u8;
}
impl EPSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPSIZE_A {
        match self.bits {
            0 => EPSIZE_A::_8_BYTE,
            1 => EPSIZE_A::_16_BYTE,
            2 => EPSIZE_A::_32_BYTE,
            3 => EPSIZE_A::_64_BYTE,
            4 => EPSIZE_A::_128_BYTE,
            5 => EPSIZE_A::_256_BYTE,
            6 => EPSIZE_A::_512_BYTE,
            7 => EPSIZE_A::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == EPSIZE_A::_8_BYTE
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == EPSIZE_A::_16_BYTE
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == EPSIZE_A::_32_BYTE
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == EPSIZE_A::_64_BYTE
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == EPSIZE_A::_128_BYTE
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == EPSIZE_A::_256_BYTE
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == EPSIZE_A::_512_BYTE
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == EPSIZE_A::_1024_BYTE
    }
}
#[doc = "Field `EPSIZE` writer - Endpoint Size"]
pub type EPSIZE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, EPSIZE_A>;
impl<'a, REG, const O: u8> EPSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut crate::W<REG> {
        self.variant(EPSIZE_A::_1024_BYTE)
    }
}
#[doc = "Field `EPDIR` reader - Endpoint Direction"]
pub type EPDIR_R = crate::BitReader<EPDIR_A>;
#[doc = "Endpoint Direction"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPDIR_A {
    #[doc = "0: The endpoint direction is OUT."]
    OUT = 0,
    #[doc = "1: The endpoint direction is IN (nor for control endpoints)."]
    IN = 1,
}
impl From<EPDIR_A> for bool {
    #[inline(always)]
    fn from(variant: EPDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl EPDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDIR_A {
        match self.bits {
            false => EPDIR_A::OUT,
            true => EPDIR_A::IN,
        }
    }
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EPDIR_A::OUT
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EPDIR_A::IN
    }
}
#[doc = "Field `EPDIR` writer - Endpoint Direction"]
pub type EPDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EPDIR_A>;
impl<'a, REG, const O: u8> EPDIR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(EPDIR_A::OUT)
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(EPDIR_A::IN)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AUTOSW_R = crate::BitReader;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AUTOSW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPTYPE` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPE_A>;
#[doc = "Endpoint Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous"]
    ISO = 1,
    #[doc = "2: Bulk"]
    BLK = 2,
    #[doc = "3: Interrupt"]
    INTRPT = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPE_A {
    type Ux = u8;
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::CTRL,
            1 => EPTYPE_A::ISO,
            2 => EPTYPE_A::BLK,
            3 => EPTYPE_A::INTRPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPE_A::CTRL
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPE_A::ISO
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == EPTYPE_A::BLK
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == EPTYPE_A::INTRPT
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type"]
pub type EPTYPE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, EPTYPE_A>;
impl<'a, REG, const O: u8> EPTYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::ISO)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::BLK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::INTRPT)
    }
}
#[doc = "Field `NBTRANS` reader - Number of transaction per microframe for isochronous endpoint"]
pub type NBTRANS_R = crate::FieldReader<NBTRANS_A>;
#[doc = "Number of transaction per microframe for isochronous endpoint"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBTRANS_A {
    #[doc = "0: reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0_TRANS = 0,
    #[doc = "1: default value: one transaction per micro-frame."]
    _1_TRANS = 1,
    #[doc = "2: 2 transactions per micro-frame. This endpoint should be configured as double-bank."]
    _2_TRANS = 2,
    #[doc = "3: 3 transactions per micro-frame. This endpoint should be configured as triple-bank."]
    _3_TRANS = 3,
}
impl From<NBTRANS_A> for u8 {
    #[inline(always)]
    fn from(variant: NBTRANS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NBTRANS_A {
    type Ux = u8;
}
impl NBTRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBTRANS_A {
        match self.bits {
            0 => NBTRANS_A::_0_TRANS,
            1 => NBTRANS_A::_1_TRANS,
            2 => NBTRANS_A::_2_TRANS,
            3 => NBTRANS_A::_3_TRANS,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn is_0_trans(&self) -> bool {
        *self == NBTRANS_A::_0_TRANS
    }
    #[doc = "default value: one transaction per micro-frame."]
    #[inline(always)]
    pub fn is_1_trans(&self) -> bool {
        *self == NBTRANS_A::_1_TRANS
    }
    #[doc = "2 transactions per micro-frame. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn is_2_trans(&self) -> bool {
        *self == NBTRANS_A::_2_TRANS
    }
    #[doc = "3 transactions per micro-frame. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn is_3_trans(&self) -> bool {
        *self == NBTRANS_A::_3_TRANS
    }
}
#[doc = "Field `NBTRANS` writer - Number of transaction per microframe for isochronous endpoint"]
pub type NBTRANS_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, NBTRANS_A>;
impl<'a, REG, const O: u8> NBTRANS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline(always)]
    pub fn _0_trans(self) -> &'a mut crate::W<REG> {
        self.variant(NBTRANS_A::_0_TRANS)
    }
    #[doc = "default value: one transaction per micro-frame."]
    #[inline(always)]
    pub fn _1_trans(self) -> &'a mut crate::W<REG> {
        self.variant(NBTRANS_A::_1_TRANS)
    }
    #[doc = "2 transactions per micro-frame. This endpoint should be configured as double-bank."]
    #[inline(always)]
    pub fn _2_trans(self) -> &'a mut crate::W<REG> {
        self.variant(NBTRANS_A::_2_TRANS)
    }
    #[doc = "3 transactions per micro-frame. This endpoint should be configured as triple-bank."]
    #[inline(always)]
    pub fn _3_trans(self) -> &'a mut crate::W<REG> {
        self.variant(NBTRANS_A::_3_TRANS)
    }
}
impl R {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    pub fn epbk(&self) -> EPBK_R {
        EPBK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    pub fn epsize(&self) -> EPSIZE_R {
        EPSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Number of transaction per microframe for isochronous endpoint"]
    #[inline(always)]
    pub fn nbtrans(&self) -> NBTRANS_R {
        NBTRANS_R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline(always)]
    #[must_use]
    pub fn alloc(&mut self) -> ALLOC_W<DEVEPTCFG_SPEC, 1> {
        ALLOC_W::new(self)
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline(always)]
    #[must_use]
    pub fn epbk(&mut self) -> EPBK_W<DEVEPTCFG_SPEC, 2> {
        EPBK_W::new(self)
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline(always)]
    #[must_use]
    pub fn epsize(&mut self) -> EPSIZE_W<DEVEPTCFG_SPEC, 4> {
        EPSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<DEVEPTCFG_SPEC, 8> {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline(always)]
    #[must_use]
    pub fn autosw(&mut self) -> AUTOSW_W<DEVEPTCFG_SPEC, 9> {
        AUTOSW_W::new(self)
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DEVEPTCFG_SPEC, 11> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 13:14 - Number of transaction per microframe for isochronous endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn nbtrans(&mut self) -> NBTRANS_W<DEVEPTCFG_SPEC, 13> {
        NBTRANS_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Device Endpoint Configuration Register (n = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deveptcfg::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deveptcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVEPTCFG_SPEC;
impl crate::RegisterSpec for DEVEPTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deveptcfg::R`](R) reader structure"]
impl crate::Readable for DEVEPTCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deveptcfg::W`](W) writer structure"]
impl crate::Writable for DEVEPTCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
