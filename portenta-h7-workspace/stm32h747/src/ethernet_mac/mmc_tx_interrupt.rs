#[doc = "Register `MMC_TX_INTERRUPT` reader"]
pub type R = crate::R<MMC_TX_INTERRUPT_SPEC>;
#[doc = "Field `TXSCOLGPIS` reader - TXSCOLGPIS"]
pub type TXSCOLGPIS_R = crate::BitReader;
#[doc = "Field `TXMCOLGPIS` reader - TXMCOLGPIS"]
pub type TXMCOLGPIS_R = crate::BitReader;
#[doc = "Field `TXGPKTIS` reader - TXGPKTIS"]
pub type TXGPKTIS_R = crate::BitReader;
#[doc = "Field `TXLPIUSCIS` reader - TXLPIUSCIS"]
pub type TXLPIUSCIS_R = crate::BitReader;
#[doc = "Field `TXLPITRCIS` reader - TXLPITRCIS"]
pub type TXLPITRCIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - TXSCOLGPIS"]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIS"]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIS"]
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIS"]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIS"]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "MMC Tx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_TX_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_SPEC {}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
