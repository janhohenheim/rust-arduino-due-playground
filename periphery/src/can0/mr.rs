#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `CANEN` reader - CAN Controller Enable"]
pub type CANEN_R = crate::BitReader;
#[doc = "Field `CANEN` writer - CAN Controller Enable"]
pub type CANEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPM` reader - Disable/Enable Low Power Mode"]
pub type LPM_R = crate::BitReader;
#[doc = "Field `LPM` writer - Disable/Enable Low Power Mode"]
pub type LPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABM` reader - Disable/Enable Autobaud/Listen mode"]
pub type ABM_R = crate::BitReader;
#[doc = "Field `ABM` writer - Disable/Enable Autobaud/Listen mode"]
pub type ABM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVL` reader - Disable/Enable Overload Frame"]
pub type OVL_R = crate::BitReader;
#[doc = "Field `OVL` writer - Disable/Enable Overload Frame"]
pub type OVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEOF` reader - Timestamp messages at each end of Frame"]
pub type TEOF_R = crate::BitReader;
#[doc = "Field `TEOF` writer - Timestamp messages at each end of Frame"]
pub type TEOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TTM` reader - Disable/Enable Time Triggered Mode"]
pub type TTM_R = crate::BitReader;
#[doc = "Field `TTM` writer - Disable/Enable Time Triggered Mode"]
pub type TTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMFRZ` reader - Enable Timer Freeze"]
pub type TIMFRZ_R = crate::BitReader;
#[doc = "Field `TIMFRZ` writer - Enable Timer Freeze"]
pub type TIMFRZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRPT` reader - Disable Repeat"]
pub type DRPT_R = crate::BitReader;
#[doc = "Field `DRPT` writer - Disable Repeat"]
pub type DRPT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXSYNC` reader - Reception Synchronization Stage (not readable)"]
pub type RXSYNC_R = crate::FieldReader<RXSYNC_A>;
#[doc = "Reception Synchronization Stage (not readable)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXSYNC_A {
    #[doc = "0: Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DOUBLE_PP = 0,
    #[doc = "1: Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DOUBLE_PN = 1,
    #[doc = "2: Rx Signal with Single Synchro Stage (Positive Edge)"]
    SINGLE_P = 2,
    #[doc = "3: Rx Signal with No Synchro Stage"]
    NONE = 3,
}
impl From<RXSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSYNC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXSYNC_A {
    type Ux = u8;
}
impl RXSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXSYNC_A> {
        match self.bits {
            0 => Some(RXSYNC_A::DOUBLE_PP),
            1 => Some(RXSYNC_A::DOUBLE_PN),
            2 => Some(RXSYNC_A::SINGLE_P),
            3 => Some(RXSYNC_A::NONE),
            _ => None,
        }
    }
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn is_double_pp(&self) -> bool {
        *self == RXSYNC_A::DOUBLE_PP
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn is_double_pn(&self) -> bool {
        *self == RXSYNC_A::DOUBLE_PN
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn is_single_p(&self) -> bool {
        *self == RXSYNC_A::SINGLE_P
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RXSYNC_A::NONE
    }
}
#[doc = "Field `RXSYNC` writer - Reception Synchronization Stage (not readable)"]
pub type RXSYNC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, RXSYNC_A>;
impl<'a, REG, const O: u8> RXSYNC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn double_pp(self) -> &'a mut crate::W<REG> {
        self.variant(RXSYNC_A::DOUBLE_PP)
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn double_pn(self) -> &'a mut crate::W<REG> {
        self.variant(RXSYNC_A::DOUBLE_PN)
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn single_p(self) -> &'a mut crate::W<REG> {
        self.variant(RXSYNC_A::SINGLE_P)
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RXSYNC_A::NONE)
    }
}
impl R {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&self) -> ABM_R {
        ABM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&self) -> OVL_R {
        OVL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&self) -> TEOF_R {
        TEOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&self) -> TTM_R {
        TTM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&self) -> TIMFRZ_R {
        TIMFRZ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&self) -> DRPT_R {
        DRPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&self) -> RXSYNC_R {
        RXSYNC_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CANEN_W<MR_SPEC, 0> {
        CANEN_W::new(self)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<MR_SPEC, 1> {
        LPM_W::new(self)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn abm(&mut self) -> ABM_W<MR_SPEC, 2> {
        ABM_W::new(self)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ovl(&mut self) -> OVL_W<MR_SPEC, 3> {
        OVL_W::new(self)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn teof(&mut self) -> TEOF_W<MR_SPEC, 4> {
        TEOF_W::new(self)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ttm(&mut self) -> TTM_W<MR_SPEC, 5> {
        TTM_W::new(self)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn timfrz(&mut self) -> TIMFRZ_W<MR_SPEC, 6> {
        TIMFRZ_W::new(self)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    #[must_use]
    pub fn drpt(&mut self) -> DRPT_W<MR_SPEC, 7> {
        DRPT_W::new(self)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    #[must_use]
    pub fn rxsync(&mut self) -> RXSYNC_W<MR_SPEC, 24> {
        RXSYNC_W::new(self)
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
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
