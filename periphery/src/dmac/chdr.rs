#[doc = "Register `CHDR` writer"]
pub type W = crate::W<CHDR_SPEC>;
#[doc = "Field `DIS0` writer - Disable \\[5:0\\]"]
pub type DIS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS1` writer - Disable \\[5:0\\]"]
pub type DIS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS2` writer - Disable \\[5:0\\]"]
pub type DIS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS3` writer - Disable \\[5:0\\]"]
pub type DIS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS4` writer - Disable \\[5:0\\]"]
pub type DIS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS5` writer - Disable \\[5:0\\]"]
pub type DIS5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RES0` writer - Resume \\[5:0\\]"]
pub type RES0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RES1` writer - Resume \\[5:0\\]"]
pub type RES1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RES2` writer - Resume \\[5:0\\]"]
pub type RES2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RES3` writer - Resume \\[5:0\\]"]
pub type RES3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RES4` writer - Resume \\[5:0\\]"]
pub type RES4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RES5` writer - Resume \\[5:0\\]"]
pub type RES5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Disable \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis0(&mut self) -> DIS0_W<CHDR_SPEC, 0> {
        DIS0_W::new(self)
    }
    #[doc = "Bit 1 - Disable \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis1(&mut self) -> DIS1_W<CHDR_SPEC, 1> {
        DIS1_W::new(self)
    }
    #[doc = "Bit 2 - Disable \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis2(&mut self) -> DIS2_W<CHDR_SPEC, 2> {
        DIS2_W::new(self)
    }
    #[doc = "Bit 3 - Disable \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis3(&mut self) -> DIS3_W<CHDR_SPEC, 3> {
        DIS3_W::new(self)
    }
    #[doc = "Bit 4 - Disable \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis4(&mut self) -> DIS4_W<CHDR_SPEC, 4> {
        DIS4_W::new(self)
    }
    #[doc = "Bit 5 - Disable \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis5(&mut self) -> DIS5_W<CHDR_SPEC, 5> {
        DIS5_W::new(self)
    }
    #[doc = "Bit 8 - Resume \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> RES0_W<CHDR_SPEC, 8> {
        RES0_W::new(self)
    }
    #[doc = "Bit 9 - Resume \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> RES1_W<CHDR_SPEC, 9> {
        RES1_W::new(self)
    }
    #[doc = "Bit 10 - Resume \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> RES2_W<CHDR_SPEC, 10> {
        RES2_W::new(self)
    }
    #[doc = "Bit 11 - Resume \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> RES3_W<CHDR_SPEC, 11> {
        RES3_W::new(self)
    }
    #[doc = "Bit 12 - Resume \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res4(&mut self) -> RES4_W<CHDR_SPEC, 12> {
        RES4_W::new(self)
    }
    #[doc = "Bit 13 - Resume \\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res5(&mut self) -> RES5_W<CHDR_SPEC, 13> {
        RES5_W::new(self)
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
#[doc = "DMAC Channel Handler Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chdr::W`](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
