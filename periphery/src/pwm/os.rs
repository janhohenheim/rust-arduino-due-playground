#[doc = "Register `OS` reader"]
pub type R = crate::R<OS_SPEC>;
#[doc = "Register `OS` writer"]
pub type W = crate::W<OS_SPEC>;
#[doc = "Field `OSH0` reader - Output Selection for PWMH output of the channel 0"]
pub type OSH0_R = crate::BitReader;
#[doc = "Field `OSH0` writer - Output Selection for PWMH output of the channel 0"]
pub type OSH0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH1` reader - Output Selection for PWMH output of the channel 1"]
pub type OSH1_R = crate::BitReader;
#[doc = "Field `OSH1` writer - Output Selection for PWMH output of the channel 1"]
pub type OSH1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH2` reader - Output Selection for PWMH output of the channel 2"]
pub type OSH2_R = crate::BitReader;
#[doc = "Field `OSH2` writer - Output Selection for PWMH output of the channel 2"]
pub type OSH2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH3` reader - Output Selection for PWMH output of the channel 3"]
pub type OSH3_R = crate::BitReader;
#[doc = "Field `OSH3` writer - Output Selection for PWMH output of the channel 3"]
pub type OSH3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH4` reader - Output Selection for PWMH output of the channel 4"]
pub type OSH4_R = crate::BitReader;
#[doc = "Field `OSH4` writer - Output Selection for PWMH output of the channel 4"]
pub type OSH4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH5` reader - Output Selection for PWMH output of the channel 5"]
pub type OSH5_R = crate::BitReader;
#[doc = "Field `OSH5` writer - Output Selection for PWMH output of the channel 5"]
pub type OSH5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH6` reader - Output Selection for PWMH output of the channel 6"]
pub type OSH6_R = crate::BitReader;
#[doc = "Field `OSH6` writer - Output Selection for PWMH output of the channel 6"]
pub type OSH6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSH7` reader - Output Selection for PWMH output of the channel 7"]
pub type OSH7_R = crate::BitReader;
#[doc = "Field `OSH7` writer - Output Selection for PWMH output of the channel 7"]
pub type OSH7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL0` reader - Output Selection for PWML output of the channel 0"]
pub type OSL0_R = crate::BitReader;
#[doc = "Field `OSL0` writer - Output Selection for PWML output of the channel 0"]
pub type OSL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL1` reader - Output Selection for PWML output of the channel 1"]
pub type OSL1_R = crate::BitReader;
#[doc = "Field `OSL1` writer - Output Selection for PWML output of the channel 1"]
pub type OSL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL2` reader - Output Selection for PWML output of the channel 2"]
pub type OSL2_R = crate::BitReader;
#[doc = "Field `OSL2` writer - Output Selection for PWML output of the channel 2"]
pub type OSL2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL3` reader - Output Selection for PWML output of the channel 3"]
pub type OSL3_R = crate::BitReader;
#[doc = "Field `OSL3` writer - Output Selection for PWML output of the channel 3"]
pub type OSL3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL4` reader - Output Selection for PWML output of the channel 4"]
pub type OSL4_R = crate::BitReader;
#[doc = "Field `OSL4` writer - Output Selection for PWML output of the channel 4"]
pub type OSL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL5` reader - Output Selection for PWML output of the channel 5"]
pub type OSL5_R = crate::BitReader;
#[doc = "Field `OSL5` writer - Output Selection for PWML output of the channel 5"]
pub type OSL5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL6` reader - Output Selection for PWML output of the channel 6"]
pub type OSL6_R = crate::BitReader;
#[doc = "Field `OSL6` writer - Output Selection for PWML output of the channel 6"]
pub type OSL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSL7` reader - Output Selection for PWML output of the channel 7"]
pub type OSL7_R = crate::BitReader;
#[doc = "Field `OSL7` writer - Output Selection for PWML output of the channel 7"]
pub type OSL7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&self) -> OSH0_R {
        OSH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&self) -> OSH1_R {
        OSH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&self) -> OSH2_R {
        OSH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&self) -> OSH3_R {
        OSH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output Selection for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn osh4(&self) -> OSH4_R {
        OSH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output Selection for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn osh5(&self) -> OSH5_R {
        OSH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output Selection for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn osh6(&self) -> OSH6_R {
        OSH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output Selection for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn osh7(&self) -> OSH7_R {
        OSH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&self) -> OSL0_R {
        OSL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&self) -> OSL1_R {
        OSL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&self) -> OSL2_R {
        OSL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&self) -> OSL3_R {
        OSL3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output Selection for PWML output of the channel 4"]
    #[inline(always)]
    pub fn osl4(&self) -> OSL4_R {
        OSL4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Selection for PWML output of the channel 5"]
    #[inline(always)]
    pub fn osl5(&self) -> OSL5_R {
        OSL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output Selection for PWML output of the channel 6"]
    #[inline(always)]
    pub fn osl6(&self) -> OSL6_R {
        OSL6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output Selection for PWML output of the channel 7"]
    #[inline(always)]
    pub fn osl7(&self) -> OSL7_R {
        OSL7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osh0(&mut self) -> OSH0_W<OS_SPEC, 0> {
        OSH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osh1(&mut self) -> OSH1_W<OS_SPEC, 1> {
        OSH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osh2(&mut self) -> OSH2_W<OS_SPEC, 2> {
        OSH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osh3(&mut self) -> OSH3_W<OS_SPEC, 3> {
        OSH3_W::new(self)
    }
    #[doc = "Bit 4 - Output Selection for PWMH output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn osh4(&mut self) -> OSH4_W<OS_SPEC, 4> {
        OSH4_W::new(self)
    }
    #[doc = "Bit 5 - Output Selection for PWMH output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn osh5(&mut self) -> OSH5_W<OS_SPEC, 5> {
        OSH5_W::new(self)
    }
    #[doc = "Bit 6 - Output Selection for PWMH output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn osh6(&mut self) -> OSH6_W<OS_SPEC, 6> {
        OSH6_W::new(self)
    }
    #[doc = "Bit 7 - Output Selection for PWMH output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn osh7(&mut self) -> OSH7_W<OS_SPEC, 7> {
        OSH7_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn osl0(&mut self) -> OSL0_W<OS_SPEC, 16> {
        OSL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn osl1(&mut self) -> OSL1_W<OS_SPEC, 17> {
        OSL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn osl2(&mut self) -> OSL2_W<OS_SPEC, 18> {
        OSL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn osl3(&mut self) -> OSL3_W<OS_SPEC, 19> {
        OSL3_W::new(self)
    }
    #[doc = "Bit 20 - Output Selection for PWML output of the channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn osl4(&mut self) -> OSL4_W<OS_SPEC, 20> {
        OSL4_W::new(self)
    }
    #[doc = "Bit 21 - Output Selection for PWML output of the channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn osl5(&mut self) -> OSL5_W<OS_SPEC, 21> {
        OSL5_W::new(self)
    }
    #[doc = "Bit 22 - Output Selection for PWML output of the channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn osl6(&mut self) -> OSL6_W<OS_SPEC, 22> {
        OSL6_W::new(self)
    }
    #[doc = "Bit 23 - Output Selection for PWML output of the channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn osl7(&mut self) -> OSL7_W<OS_SPEC, 23> {
        OSL7_W::new(self)
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
#[doc = "PWM Output Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`os::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`os::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OS_SPEC;
impl crate::RegisterSpec for OS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`os::R`](R) reader structure"]
impl crate::Readable for OS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`os::W`](W) writer structure"]
impl crate::Writable for OS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OS to value 0"]
impl crate::Resettable for OS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
