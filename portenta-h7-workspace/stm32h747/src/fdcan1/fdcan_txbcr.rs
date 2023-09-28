#[doc = "Register `FDCAN_TXBCR` reader"]
pub type R = crate::R<FDCAN_TXBCR_SPEC>;
#[doc = "Register `FDCAN_TXBCR` writer"]
pub type W = crate::W<FDCAN_TXBCR_SPEC>;
#[doc = "Field `CR` reader - Cancellation Request"]
pub type CR_R = crate::FieldReader<u32>;
#[doc = "Field `CR` writer - Cancellation Request"]
pub type CR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Request"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Request"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<FDCAN_TXBCR_SPEC, 0> {
        CR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBCR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcr::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbcr::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCR to value 0"]
impl crate::Resettable for FDCAN_TXBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
