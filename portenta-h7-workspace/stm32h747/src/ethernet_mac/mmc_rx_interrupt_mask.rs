#[doc = "Register `MMC_RX_INTERRUPT_MASK` reader"]
pub type R = crate::R<MMC_RX_INTERRUPT_MASK_SPEC>;
#[doc = "Register `MMC_RX_INTERRUPT_MASK` writer"]
pub type W = crate::W<MMC_RX_INTERRUPT_MASK_SPEC>;
#[doc = "Field `RXCRCERPIM` reader - RXCRCERPIM"]
pub type RXCRCERPIM_R = crate::BitReader;
#[doc = "Field `RXCRCERPIM` writer - RXCRCERPIM"]
pub type RXCRCERPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXALGNERPIM` reader - RXALGNERPIM"]
pub type RXALGNERPIM_R = crate::BitReader;
#[doc = "Field `RXALGNERPIM` writer - RXALGNERPIM"]
pub type RXALGNERPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUCGPIM` reader - RXUCGPIM"]
pub type RXUCGPIM_R = crate::BitReader;
#[doc = "Field `RXUCGPIM` writer - RXUCGPIM"]
pub type RXUCGPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXLPIUSCIM` reader - RXLPIUSCIM"]
pub type RXLPIUSCIM_R = crate::BitReader;
#[doc = "Field `RXLPIUSCIM` writer - RXLPIUSCIM"]
pub type RXLPIUSCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXLPITRCIM` reader - RXLPITRCIM"]
pub type RXLPITRCIM_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RXCRCERPIM_R {
        RXCRCERPIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RXALGNERPIM_R {
        RXALGNERPIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    pub fn rxucgpim(&self) -> RXUCGPIM_R {
        RXUCGPIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RXLPIUSCIM_R {
        RXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIM"]
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RXLPITRCIM_R {
        RXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerpim(&mut self) -> RXCRCERPIM_W<MMC_RX_INTERRUPT_MASK_SPEC, 5> {
        RXCRCERPIM_W::new(self)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxalgnerpim(&mut self) -> RXALGNERPIM_W<MMC_RX_INTERRUPT_MASK_SPEC, 6> {
        RXALGNERPIM_W::new(self)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxucgpim(&mut self) -> RXUCGPIM_W<MMC_RX_INTERRUPT_MASK_SPEC, 17> {
        RXUCGPIM_W::new(self)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiuscim(&mut self) -> RXLPIUSCIM_W<MMC_RX_INTERRUPT_MASK_SPEC, 26> {
        RXLPIUSCIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Rx interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_RX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_rx_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MMC_RX_INTERRUPT_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
