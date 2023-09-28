#[doc = "Register `MMC_RX_INTERRUPT` reader"]
pub type R = crate::R<MMC_RX_INTERRUPT_SPEC>;
#[doc = "Field `RXCRCERPIS` reader - RXCRCERPIS"]
pub type RXCRCERPIS_R = crate::BitReader;
#[doc = "Field `RXALGNERPIS` reader - RXALGNERPIS"]
pub type RXALGNERPIS_R = crate::BitReader;
#[doc = "Field `RXUCGPIS` reader - RXUCGPIS"]
pub type RXUCGPIS_R = crate::BitReader;
#[doc = "Field `RXLPIUSCIS` reader - RXLPIUSCIS"]
pub type RXLPIUSCIS_R = crate::BitReader;
#[doc = "Field `RXLPITRCIS` reader - RXLPITRCIS"]
pub type RXLPITRCIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - RXCRCERPIS"]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIS"]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIS"]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIS"]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIS"]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "MMC Rx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_RX_INTERRUPT_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_SPEC {}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
