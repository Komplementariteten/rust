#[doc = "Register `DSI_PSR` reader"]
pub type R = crate::R<DSI_PSR_SPEC>;
#[doc = "Field `PD` reader - PD"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PSSC` reader - PSSC"]
pub type PSSC_R = crate::BitReader;
#[doc = "Field `UANC` reader - UANC"]
pub type UANC_R = crate::BitReader;
#[doc = "Field `PSS0` reader - PSS0"]
pub type PSS0_R = crate::BitReader;
#[doc = "Field `UAN0` reader - UAN0"]
pub type UAN0_R = crate::BitReader;
#[doc = "Field `RUE0` reader - RUE0"]
pub type RUE0_R = crate::BitReader;
#[doc = "Field `PSS1` reader - PSS1"]
pub type PSS1_R = crate::BitReader;
#[doc = "Field `UAN1` reader - UAN1"]
pub type UAN1_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSSC"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UANC"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSS0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UAN0"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RUE0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSS1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UAN1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_psr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PSR_SPEC;
impl crate::RegisterSpec for DSI_PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_psr::R`](R) reader structure"]
impl crate::Readable for DSI_PSR_SPEC {}
#[doc = "`reset()` method sets DSI_PSR to value 0x1528"]
impl crate::Resettable for DSI_PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1528;
}
