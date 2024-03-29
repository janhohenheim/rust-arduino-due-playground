#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Field `DRDSTATE` reader - Dual Role Device State"]
pub type DRDSTATE_R = crate::FieldReader<DRDSTATE_A>;
#[doc = "Dual Role Device State\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRDSTATE_A {
    #[doc = "0: This is the start state for A-devices (when the ID pin is 0)"]
    A_IDLESTATE = 0,
    #[doc = "1: In this state, the A-device waits for the voltage on VBus to rise above the A-device VBus Valid threshold (4.4 V)."]
    A_WAIT_VRISE = 1,
    #[doc = "2: In this state, the A-device waits for the B-device to signal a connection."]
    A_WAIT_BCON = 2,
    #[doc = "3: In this state, the A-device that operates in Host mode is operational."]
    A_HOST = 3,
    #[doc = "4: The A-device operating as a host is in the suspend mode."]
    A_SUSPEND = 4,
    #[doc = "5: The A-device operates as a peripheral."]
    A_PERIPHERAL = 5,
    #[doc = "6: In this state, the A-device waits for the voltage on VBus to drop below the A-device Session Valid threshold (1.4 V)."]
    A_WAIT_VFALL = 6,
    #[doc = "7: In this state, the A-device waits for recovery of the over-current condition that caused it to enter this state."]
    A_VBUS_ERR = 7,
    #[doc = "8: In this state, the A-device waits for the data USB line to discharge (100 us)."]
    A_WAIT_DISCHARGE = 8,
    #[doc = "9: This is the start state for B-device (when the ID pin is 1)."]
    B_IDLE = 9,
    #[doc = "10: In this state, the B-device acts as the peripheral."]
    B_PERIPHERAL = 10,
    #[doc = "11: In this state, the B-device is in suspend mode and waits until 3 ms before initiating the HNP protocol if requested."]
    B_WAIT_BEGIN_HNP = 11,
    #[doc = "12: In this state, the B-device waits for the data USB line to discharge (100 us) before becoming Host."]
    B_WAIT_DISCHARGE = 12,
    #[doc = "13: In this state, the B-device waits for the A-device to signal a connect before becoming B-Host."]
    B_WAIT_ACON = 13,
    #[doc = "14: In this state, the B-device acts as the Host."]
    B_HOST = 14,
    #[doc = "15: In this state, the B-device attempts to start a session using the SRP protocol."]
    B_SRP_INIT = 15,
}
impl From<DRDSTATE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRDSTATE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRDSTATE_A {
    type Ux = u8;
}
impl DRDSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDSTATE_A {
        match self.bits {
            0 => DRDSTATE_A::A_IDLESTATE,
            1 => DRDSTATE_A::A_WAIT_VRISE,
            2 => DRDSTATE_A::A_WAIT_BCON,
            3 => DRDSTATE_A::A_HOST,
            4 => DRDSTATE_A::A_SUSPEND,
            5 => DRDSTATE_A::A_PERIPHERAL,
            6 => DRDSTATE_A::A_WAIT_VFALL,
            7 => DRDSTATE_A::A_VBUS_ERR,
            8 => DRDSTATE_A::A_WAIT_DISCHARGE,
            9 => DRDSTATE_A::B_IDLE,
            10 => DRDSTATE_A::B_PERIPHERAL,
            11 => DRDSTATE_A::B_WAIT_BEGIN_HNP,
            12 => DRDSTATE_A::B_WAIT_DISCHARGE,
            13 => DRDSTATE_A::B_WAIT_ACON,
            14 => DRDSTATE_A::B_HOST,
            15 => DRDSTATE_A::B_SRP_INIT,
            _ => unreachable!(),
        }
    }
    #[doc = "This is the start state for A-devices (when the ID pin is 0)"]
    #[inline(always)]
    pub fn is_a_idlestate(&self) -> bool {
        *self == DRDSTATE_A::A_IDLESTATE
    }
    #[doc = "In this state, the A-device waits for the voltage on VBus to rise above the A-device VBus Valid threshold (4.4 V)."]
    #[inline(always)]
    pub fn is_a_wait_vrise(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_VRISE
    }
    #[doc = "In this state, the A-device waits for the B-device to signal a connection."]
    #[inline(always)]
    pub fn is_a_wait_bcon(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_BCON
    }
    #[doc = "In this state, the A-device that operates in Host mode is operational."]
    #[inline(always)]
    pub fn is_a_host(&self) -> bool {
        *self == DRDSTATE_A::A_HOST
    }
    #[doc = "The A-device operating as a host is in the suspend mode."]
    #[inline(always)]
    pub fn is_a_suspend(&self) -> bool {
        *self == DRDSTATE_A::A_SUSPEND
    }
    #[doc = "The A-device operates as a peripheral."]
    #[inline(always)]
    pub fn is_a_peripheral(&self) -> bool {
        *self == DRDSTATE_A::A_PERIPHERAL
    }
    #[doc = "In this state, the A-device waits for the voltage on VBus to drop below the A-device Session Valid threshold (1.4 V)."]
    #[inline(always)]
    pub fn is_a_wait_vfall(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_VFALL
    }
    #[doc = "In this state, the A-device waits for recovery of the over-current condition that caused it to enter this state."]
    #[inline(always)]
    pub fn is_a_vbus_err(&self) -> bool {
        *self == DRDSTATE_A::A_VBUS_ERR
    }
    #[doc = "In this state, the A-device waits for the data USB line to discharge (100 us)."]
    #[inline(always)]
    pub fn is_a_wait_discharge(&self) -> bool {
        *self == DRDSTATE_A::A_WAIT_DISCHARGE
    }
    #[doc = "This is the start state for B-device (when the ID pin is 1)."]
    #[inline(always)]
    pub fn is_b_idle(&self) -> bool {
        *self == DRDSTATE_A::B_IDLE
    }
    #[doc = "In this state, the B-device acts as the peripheral."]
    #[inline(always)]
    pub fn is_b_peripheral(&self) -> bool {
        *self == DRDSTATE_A::B_PERIPHERAL
    }
    #[doc = "In this state, the B-device is in suspend mode and waits until 3 ms before initiating the HNP protocol if requested."]
    #[inline(always)]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        *self == DRDSTATE_A::B_WAIT_BEGIN_HNP
    }
    #[doc = "In this state, the B-device waits for the data USB line to discharge (100 us) before becoming Host."]
    #[inline(always)]
    pub fn is_b_wait_discharge(&self) -> bool {
        *self == DRDSTATE_A::B_WAIT_DISCHARGE
    }
    #[doc = "In this state, the B-device waits for the A-device to signal a connect before becoming B-Host."]
    #[inline(always)]
    pub fn is_b_wait_acon(&self) -> bool {
        *self == DRDSTATE_A::B_WAIT_ACON
    }
    #[doc = "In this state, the B-device acts as the Host."]
    #[inline(always)]
    pub fn is_b_host(&self) -> bool {
        *self == DRDSTATE_A::B_HOST
    }
    #[doc = "In this state, the B-device attempts to start a session using the SRP protocol."]
    #[inline(always)]
    pub fn is_b_srp_init(&self) -> bool {
        *self == DRDSTATE_A::B_SRP_INIT
    }
}
impl R {
    #[doc = "Bits 0:3 - Dual Role Device State"]
    #[inline(always)]
    pub fn drdstate(&self) -> DRDSTATE_R {
        DRDSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "General Finite State Machine Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`reset()` method sets FSM to value 0x09"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
