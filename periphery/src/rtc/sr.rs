#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `ACKUPD` reader - Acknowledge for Update"]
pub type ACKUPD_R = crate::BitReader<ACKUPD_A>;
#[doc = "Acknowledge for Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACKUPD_A {
    #[doc = "0: Time and calendar registers cannot be updated."]
    FREERUN = 0,
    #[doc = "1: Time and calendar registers can be updated."]
    UPDATE = 1,
}
impl From<ACKUPD_A> for bool {
    #[inline(always)]
    fn from(variant: ACKUPD_A) -> Self {
        variant as u8 != 0
    }
}
impl ACKUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKUPD_A {
        match self.bits {
            false => ACKUPD_A::FREERUN,
            true => ACKUPD_A::UPDATE,
        }
    }
    #[doc = "Time and calendar registers cannot be updated."]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPD_A::FREERUN
    }
    #[doc = "Time and calendar registers can be updated."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ACKUPD_A::UPDATE
    }
}
#[doc = "Field `ALARM` reader - Alarm Flag"]
pub type ALARM_R = crate::BitReader<ALARM_A>;
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM_A {
    #[doc = "0: No alarm matching condition occurred."]
    NO_ALARMEVENT = 0,
    #[doc = "1: An alarm matching condition has occurred."]
    ALARMEVENT = 1,
}
impl From<ALARM_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_A {
        match self.bits {
            false => ALARM_A::NO_ALARMEVENT,
            true => ALARM_A::ALARMEVENT,
        }
    }
    #[doc = "No alarm matching condition occurred."]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARM_A::NO_ALARMEVENT
    }
    #[doc = "An alarm matching condition has occurred."]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == ALARM_A::ALARMEVENT
    }
}
#[doc = "Field `SEC` reader - Second Event"]
pub type SEC_R = crate::BitReader<SEC_A>;
#[doc = "Second Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEC_A {
    #[doc = "0: No second event has occurred since the last clear."]
    NO_SECEVENT = 0,
    #[doc = "1: At least one second event has occurred since the last clear."]
    SECEVENT = 1,
}
impl From<SEC_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as u8 != 0
    }
}
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            false => SEC_A::NO_SECEVENT,
            true => SEC_A::SECEVENT,
        }
    }
    #[doc = "No second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == SEC_A::NO_SECEVENT
    }
    #[doc = "At least one second event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == SEC_A::SECEVENT
    }
}
#[doc = "Field `TIMEV` reader - Time Event"]
pub type TIMEV_R = crate::BitReader<TIMEV_A>;
#[doc = "Time Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEV_A {
    #[doc = "0: No time event has occurred since the last clear."]
    NO_TIMEVENT = 0,
    #[doc = "1: At least one time event has occurred since the last clear."]
    TIMEVENT = 1,
}
impl From<TIMEV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEV_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEV_A {
        match self.bits {
            false => TIMEV_A::NO_TIMEVENT,
            true => TIMEV_A::TIMEVENT,
        }
    }
    #[doc = "No time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEV_A::NO_TIMEVENT
    }
    #[doc = "At least one time event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == TIMEV_A::TIMEVENT
    }
}
#[doc = "Field `CALEV` reader - Calendar Event"]
pub type CALEV_R = crate::BitReader<CALEV_A>;
#[doc = "Calendar Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEV_A {
    #[doc = "0: No calendar event has occurred since the last clear."]
    NO_CALEVENT = 0,
    #[doc = "1: At least one calendar event has occurred since the last clear."]
    CALEVENT = 1,
}
impl From<CALEV_A> for bool {
    #[inline(always)]
    fn from(variant: CALEV_A) -> Self {
        variant as u8 != 0
    }
}
impl CALEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALEV_A {
        match self.bits {
            false => CALEV_A::NO_CALEVENT,
            true => CALEV_A::CALEVENT,
        }
    }
    #[doc = "No calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEV_A::NO_CALEVENT
    }
    #[doc = "At least one calendar event has occurred since the last clear."]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == CALEV_A::CALEVENT
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
