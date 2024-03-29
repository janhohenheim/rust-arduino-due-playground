#[doc = "Register `CTRLA0` reader"]
pub type R = crate::R<CTRLA0_SPEC>;
#[doc = "Register `CTRLA0` writer"]
pub type W = crate::W<CTRLA0_SPEC>;
#[doc = "Field `BTSIZE` reader - Buffer Transfer Size"]
pub type BTSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `BTSIZE` writer - Buffer Transfer Size"]
pub type BTSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `SCSIZE` reader - Source Chunk Transfer Size."]
pub type SCSIZE_R = crate::FieldReader<SCSIZE_A>;
#[doc = "Source Chunk Transfer Size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 4 data transferred"]
    CHK_4 = 1,
    #[doc = "2: 8 data transferred"]
    CHK_8 = 2,
    #[doc = "3: 16 data transferred"]
    CHK_16 = 3,
}
impl From<SCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCSIZE_A {
    type Ux = u8;
}
impl SCSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCSIZE_A> {
        match self.bits {
            0 => Some(SCSIZE_A::CHK_1),
            1 => Some(SCSIZE_A::CHK_4),
            2 => Some(SCSIZE_A::CHK_8),
            3 => Some(SCSIZE_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == SCSIZE_A::CHK_1
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == SCSIZE_A::CHK_4
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == SCSIZE_A::CHK_8
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == SCSIZE_A::CHK_16
    }
}
#[doc = "Field `SCSIZE` writer - Source Chunk Transfer Size."]
pub type SCSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SCSIZE_A>;
impl<'a, REG, const O: u8> SCSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut crate::W<REG> {
        self.variant(SCSIZE_A::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut crate::W<REG> {
        self.variant(SCSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut crate::W<REG> {
        self.variant(SCSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut crate::W<REG> {
        self.variant(SCSIZE_A::CHK_16)
    }
}
#[doc = "Field `DCSIZE` reader - Destination Chunk Transfer Size"]
pub type DCSIZE_R = crate::FieldReader<DCSIZE_A>;
#[doc = "Destination Chunk Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 4 data transferred"]
    CHK_4 = 1,
    #[doc = "2: 8 data transferred"]
    CHK_8 = 2,
    #[doc = "3: 16 data transferred"]
    CHK_16 = 3,
}
impl From<DCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DCSIZE_A {
    type Ux = u8;
}
impl DCSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCSIZE_A> {
        match self.bits {
            0 => Some(DCSIZE_A::CHK_1),
            1 => Some(DCSIZE_A::CHK_4),
            2 => Some(DCSIZE_A::CHK_8),
            3 => Some(DCSIZE_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == DCSIZE_A::CHK_1
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == DCSIZE_A::CHK_4
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == DCSIZE_A::CHK_8
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == DCSIZE_A::CHK_16
    }
}
#[doc = "Field `DCSIZE` writer - Destination Chunk Transfer Size"]
pub type DCSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, DCSIZE_A>;
impl<'a, REG, const O: u8> DCSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut crate::W<REG> {
        self.variant(DCSIZE_A::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut crate::W<REG> {
        self.variant(DCSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut crate::W<REG> {
        self.variant(DCSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut crate::W<REG> {
        self.variant(DCSIZE_A::CHK_16)
    }
}
#[doc = "Field `SRC_WIDTH` reader - Transfer Width for the Source"]
pub type SRC_WIDTH_R = crate::FieldReader<SRC_WIDTH_A>;
#[doc = "Transfer Width for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_WIDTH_A {
    #[doc = "0: the transfer size is set to 8-bit width"]
    BYTE = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HALF_WORD = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    WORD = 2,
}
impl From<SRC_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRC_WIDTH_A {
    type Ux = u8;
}
impl SRC_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_WIDTH_A> {
        match self.bits {
            0 => Some(SRC_WIDTH_A::BYTE),
            1 => Some(SRC_WIDTH_A::HALF_WORD),
            2 => Some(SRC_WIDTH_A::WORD),
            _ => None,
        }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SRC_WIDTH_A::BYTE
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRC_WIDTH_A::HALF_WORD
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRC_WIDTH_A::WORD
    }
}
#[doc = "Field `SRC_WIDTH` writer - Transfer Width for the Source"]
pub type SRC_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SRC_WIDTH_A>;
impl<'a, REG, const O: u8> SRC_WIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_WIDTH_A::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_WIDTH_A::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SRC_WIDTH_A::WORD)
    }
}
#[doc = "Field `DST_WIDTH` reader - Transfer Width for the Destination"]
pub type DST_WIDTH_R = crate::FieldReader<DST_WIDTH_A>;
#[doc = "Transfer Width for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DST_WIDTH_A {
    #[doc = "0: the transfer size is set to 8-bit width"]
    BYTE = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HALF_WORD = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    WORD = 2,
}
impl From<DST_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DST_WIDTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DST_WIDTH_A {
    type Ux = u8;
}
impl DST_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DST_WIDTH_A> {
        match self.bits {
            0 => Some(DST_WIDTH_A::BYTE),
            1 => Some(DST_WIDTH_A::HALF_WORD),
            2 => Some(DST_WIDTH_A::WORD),
            _ => None,
        }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DST_WIDTH_A::BYTE
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DST_WIDTH_A::HALF_WORD
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DST_WIDTH_A::WORD
    }
}
#[doc = "Field `DST_WIDTH` writer - Transfer Width for the Destination"]
pub type DST_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DST_WIDTH_A>;
impl<'a, REG, const O: u8> DST_WIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DST_WIDTH_A::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(DST_WIDTH_A::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(DST_WIDTH_A::WORD)
    }
}
#[doc = "Field `DONE` reader - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
pub type DONE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&self) -> BTSIZE_R {
        BTSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&self) -> SCSIZE_R {
        SCSIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&self) -> DCSIZE_R {
        DCSIZE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&self) -> SRC_WIDTH_R {
        SRC_WIDTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&self) -> DST_WIDTH_R {
        DST_WIDTH_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn btsize(&mut self) -> BTSIZE_W<CTRLA0_SPEC, 0> {
        BTSIZE_W::new(self)
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    #[must_use]
    pub fn scsize(&mut self) -> SCSIZE_W<CTRLA0_SPEC, 16> {
        SCSIZE_W::new(self)
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn dcsize(&mut self) -> DCSIZE_W<CTRLA0_SPEC, 20> {
        DCSIZE_W::new(self)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    #[must_use]
    pub fn src_width(&mut self) -> SRC_WIDTH_W<CTRLA0_SPEC, 24> {
        SRC_WIDTH_W::new(self)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    #[must_use]
    pub fn dst_width(&mut self) -> DST_WIDTH_W<CTRLA0_SPEC, 28> {
        DST_WIDTH_W::new(self)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<CTRLA0_SPEC, 31> {
        DONE_W::new(self)
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
#[doc = "DMAC Channel Control A Register (ch_num = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrla0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrla0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLA0_SPEC;
impl crate::RegisterSpec for CTRLA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla0::R`](R) reader structure"]
impl crate::Readable for CTRLA0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrla0::W`](W) writer structure"]
impl crate::Writable for CTRLA0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLA0 to value 0"]
impl crate::Resettable for CTRLA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
