#[doc = "Register `SMMR` reader"]
pub type R = crate::R<SMMR_SPEC>;
#[doc = "Register `SMMR` writer"]
pub type W = crate::W<SMMR_SPEC>;
#[doc = "Field `SMTH` reader - Supply Monitor Threshold"]
pub type SMTH_R = crate::FieldReader;
#[doc = "Field `SMTH` writer - Supply Monitor Threshold"]
pub type SMTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SMSMPL` reader - Supply Monitor Sampling Period"]
pub type SMSMPL_R = crate::FieldReader<SMSMPL_A>;
#[doc = "Supply Monitor Sampling Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMSMPL_A {
    #[doc = "0: Supply Monitor disabled"]
    SMD = 0,
    #[doc = "1: Continuous Supply Monitor"]
    CSM = 1,
    #[doc = "2: Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK = 2,
    #[doc = "3: Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK = 3,
    #[doc = "4: Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK = 4,
}
impl From<SMSMPL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMSMPL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMSMPL_A {
    type Ux = u8;
}
impl SMSMPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMSMPL_A> {
        match self.bits {
            0 => Some(SMSMPL_A::SMD),
            1 => Some(SMSMPL_A::CSM),
            2 => Some(SMSMPL_A::_32SLCK),
            3 => Some(SMSMPL_A::_256SLCK),
            4 => Some(SMSMPL_A::_2048SLCK),
            _ => None,
        }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn is_smd(&self) -> bool {
        *self == SMSMPL_A::SMD
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn is_csm(&self) -> bool {
        *self == SMSMPL_A::CSM
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn is_32slck(&self) -> bool {
        *self == SMSMPL_A::_32SLCK
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn is_256slck(&self) -> bool {
        *self == SMSMPL_A::_256SLCK
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn is_2048slck(&self) -> bool {
        *self == SMSMPL_A::_2048SLCK
    }
}
#[doc = "Field `SMSMPL` writer - Supply Monitor Sampling Period"]
pub type SMSMPL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SMSMPL_A>;
impl<'a, REG, const O: u8> SMSMPL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Supply Monitor disabled"]
    #[inline(always)]
    pub fn smd(self) -> &'a mut crate::W<REG> {
        self.variant(SMSMPL_A::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline(always)]
    pub fn csm(self) -> &'a mut crate::W<REG> {
        self.variant(SMSMPL_A::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline(always)]
    pub fn _32slck(self) -> &'a mut crate::W<REG> {
        self.variant(SMSMPL_A::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline(always)]
    pub fn _256slck(self) -> &'a mut crate::W<REG> {
        self.variant(SMSMPL_A::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline(always)]
    pub fn _2048slck(self) -> &'a mut crate::W<REG> {
        self.variant(SMSMPL_A::_2048SLCK)
    }
}
#[doc = "Field `SMRSTEN` reader - Supply Monitor Reset Enable"]
pub type SMRSTEN_R = crate::BitReader<SMRSTEN_A>;
#[doc = "Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMRSTEN_A {
    #[doc = "0: the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE = 1,
}
impl From<SMRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTEN_A {
        match self.bits {
            false => SMRSTEN_A::NOT_ENABLE,
            true => SMRSTEN_A::ENABLE,
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMRSTEN_A::NOT_ENABLE
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMRSTEN_A::ENABLE
    }
}
#[doc = "Field `SMRSTEN` writer - Supply Monitor Reset Enable"]
pub type SMRSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMRSTEN_A>;
impl<'a, REG, const O: u8> SMRSTEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMRSTEN_A::NOT_ENABLE)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMRSTEN_A::ENABLE)
    }
}
#[doc = "Field `SMIEN` reader - Supply Monitor Interrupt Enable"]
pub type SMIEN_R = crate::BitReader<SMIEN_A>;
#[doc = "Supply Monitor Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMIEN_A {
    #[doc = "0: the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE = 1,
}
impl From<SMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIEN_A {
        match self.bits {
            false => SMIEN_A::NOT_ENABLE,
            true => SMIEN_A::ENABLE,
        }
    }
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMIEN_A::NOT_ENABLE
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMIEN_A::ENABLE
    }
}
#[doc = "Field `SMIEN` writer - Supply Monitor Interrupt Enable"]
pub type SMIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SMIEN_A>;
impl<'a, REG, const O: u8> SMIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMIEN_A::NOT_ENABLE)
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SMIEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    pub fn smth(&self) -> SMTH_R {
        SMTH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    pub fn smsmpl(&self) -> SMSMPL_R {
        SMSMPL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn smrsten(&self) -> SMRSTEN_R {
        SMRSTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn smien(&self) -> SMIEN_R {
        SMIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn smth(&mut self) -> SMTH_W<SMMR_SPEC, 0> {
        SMTH_W::new(self)
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline(always)]
    #[must_use]
    pub fn smsmpl(&mut self) -> SMSMPL_W<SMMR_SPEC, 8> {
        SMSMPL_W::new(self)
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smrsten(&mut self) -> SMRSTEN_W<SMMR_SPEC, 12> {
        SMRSTEN_W::new(self)
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smien(&mut self) -> SMIEN_W<SMMR_SPEC, 13> {
        SMIEN_W::new(self)
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
#[doc = "Supply Controller Supply Monitor Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMMR_SPEC;
impl crate::RegisterSpec for SMMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smmr::R`](R) reader structure"]
impl crate::Readable for SMMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smmr::W`](W) writer structure"]
impl crate::Writable for SMMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMMR to value 0"]
impl crate::Resettable for SMMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
