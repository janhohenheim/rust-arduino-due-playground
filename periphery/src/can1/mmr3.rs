#[doc = "Register `MMR3` reader"]
pub type R = crate::R<MMR3_SPEC>;
#[doc = "Register `MMR3` writer"]
pub type W = crate::W<MMR3_SPEC>;
#[doc = "Field `MTIMEMARK` reader - Mailbox Timemark"]
pub type MTIMEMARK_R = crate::FieldReader<u16>;
#[doc = "Field `MTIMEMARK` writer - Mailbox Timemark"]
pub type MTIMEMARK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `PRIOR` reader - Mailbox Priority"]
pub type PRIOR_R = crate::FieldReader;
#[doc = "Field `PRIOR` writer - Mailbox Priority"]
pub type PRIOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MOT` reader - Mailbox Object Type"]
pub type MOT_R = crate::FieldReader<MOT_A>;
#[doc = "Mailbox Object Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOT_A {
    #[doc = "0: Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MB_DISABLED = 0,
    #[doc = "1: Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MB_RX = 1,
    #[doc = "2: Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MB_RX_OVERWRITE = 2,
    #[doc = "3: Transmit mailbox. Mailbox is configured for transmission."]
    MB_TX = 3,
    #[doc = "4: Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MB_CONSUMER = 4,
    #[doc = "5: Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MB_PRODUCER = 5,
}
impl From<MOT_A> for u8 {
    #[inline(always)]
    fn from(variant: MOT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MOT_A {
    type Ux = u8;
}
impl MOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MOT_A> {
        match self.bits {
            0 => Some(MOT_A::MB_DISABLED),
            1 => Some(MOT_A::MB_RX),
            2 => Some(MOT_A::MB_RX_OVERWRITE),
            3 => Some(MOT_A::MB_TX),
            4 => Some(MOT_A::MB_CONSUMER),
            5 => Some(MOT_A::MB_PRODUCER),
            _ => None,
        }
    }
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn is_mb_disabled(&self) -> bool {
        *self == MOT_A::MB_DISABLED
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn is_mb_rx(&self) -> bool {
        *self == MOT_A::MB_RX
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn is_mb_rx_overwrite(&self) -> bool {
        *self == MOT_A::MB_RX_OVERWRITE
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn is_mb_tx(&self) -> bool {
        *self == MOT_A::MB_TX
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn is_mb_consumer(&self) -> bool {
        *self == MOT_A::MB_CONSUMER
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn is_mb_producer(&self) -> bool {
        *self == MOT_A::MB_PRODUCER
    }
}
#[doc = "Field `MOT` writer - Mailbox Object Type"]
pub type MOT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MOT_A>;
impl<'a, REG, const O: u8> MOT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn mb_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOT_A::MB_DISABLED)
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn mb_rx(self) -> &'a mut crate::W<REG> {
        self.variant(MOT_A::MB_RX)
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn mb_rx_overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(MOT_A::MB_RX_OVERWRITE)
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn mb_tx(self) -> &'a mut crate::W<REG> {
        self.variant(MOT_A::MB_TX)
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn mb_consumer(self) -> &'a mut crate::W<REG> {
        self.variant(MOT_A::MB_CONSUMER)
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn mb_producer(self) -> &'a mut crate::W<REG> {
        self.variant(MOT_A::MB_PRODUCER)
    }
}
impl R {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    pub fn mtimemark(&self) -> MTIMEMARK_R {
        MTIMEMARK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    pub fn prior(&self) -> PRIOR_R {
        PRIOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    pub fn mot(&self) -> MOT_R {
        MOT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    #[must_use]
    pub fn mtimemark(&mut self) -> MTIMEMARK_W<MMR3_SPEC, 0> {
        MTIMEMARK_W::new(self)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    #[must_use]
    pub fn prior(&mut self) -> PRIOR_W<MMR3_SPEC, 16> {
        PRIOR_W::new(self)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    #[must_use]
    pub fn mot(&mut self) -> MOT_W<MMR3_SPEC, 24> {
        MOT_W::new(self)
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
#[doc = "Mailbox Mode Register (MB = 3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMR3_SPEC;
impl crate::RegisterSpec for MMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr3::R`](R) reader structure"]
impl crate::Readable for MMR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmr3::W`](W) writer structure"]
impl crate::Writable for MMR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMR3 to value 0"]
impl crate::Resettable for MMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
