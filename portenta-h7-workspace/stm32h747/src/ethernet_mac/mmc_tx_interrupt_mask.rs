#[doc = "Register `MMC_TX_INTERRUPT_MASK` reader"]
pub type R = crate::R<MMC_TX_INTERRUPT_MASK_SPEC>;
#[doc = "Register `MMC_TX_INTERRUPT_MASK` writer"]
pub type W = crate::W<MMC_TX_INTERRUPT_MASK_SPEC>;
#[doc = "Field `TXSCOLGPIM` reader - TXSCOLGPIM"]
pub type TXSCOLGPIM_R = crate::BitReader;
#[doc = "Field `TXSCOLGPIM` writer - TXSCOLGPIM"]
pub type TXSCOLGPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXMCOLGPIM` reader - TXMCOLGPIM"]
pub type TXMCOLGPIM_R = crate::BitReader;
#[doc = "Field `TXMCOLGPIM` writer - TXMCOLGPIM"]
pub type TXMCOLGPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXGPKTIM` reader - TXGPKTIM"]
pub type TXGPKTIM_R = crate::BitReader;
#[doc = "Field `TXGPKTIM` writer - TXGPKTIM"]
pub type TXGPKTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXLPIUSCIM` reader - TXLPIUSCIM"]
pub type TXLPIUSCIM_R = crate::BitReader;
#[doc = "Field `TXLPIUSCIM` writer - TXLPIUSCIM"]
pub type TXLPIUSCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXLPITRCIM` reader - TXLPITRCIM"]
pub type TXLPITRCIM_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - TXSCOLGPIM"]
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIM"]
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIM"]
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIM"]
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIM"]
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - TXSCOLGPIM"]
    #[inline(always)]
    #[must_use]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W<MMC_TX_INTERRUPT_MASK_SPEC, 14> {
        TXSCOLGPIM_W::new(self)
    }
    #[doc = "Bit 15 - TXMCOLGPIM"]
    #[inline(always)]
    #[must_use]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W<MMC_TX_INTERRUPT_MASK_SPEC, 15> {
        TXMCOLGPIM_W::new(self)
    }
    #[doc = "Bit 21 - TXGPKTIM"]
    #[inline(always)]
    #[must_use]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W<MMC_TX_INTERRUPT_MASK_SPEC, 21> {
        TXGPKTIM_W::new(self)
    }
    #[doc = "Bit 26 - TXLPIUSCIM"]
    #[inline(always)]
    #[must_use]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W<MMC_TX_INTERRUPT_MASK_SPEC, 26> {
        TXLPIUSCIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MMC Tx interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_TX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_tx_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MMC_TX_INTERRUPT_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
