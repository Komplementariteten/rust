#[doc = "Register `FDCAN_TXBAR` reader"]
pub type R = crate::R<FDCAN_TXBAR_SPEC>;
#[doc = "Register `FDCAN_TXBAR` writer"]
pub type W = crate::W<FDCAN_TXBAR_SPEC>;
#[doc = "Field `AR` reader - Add Request"]
pub type AR_R = crate::FieldReader<u32>;
#[doc = "Field `AR` writer - Add Request"]
pub type AR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Add Request"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Add Request"]
    #[inline(always)]
    #[must_use]
    pub fn ar(&mut self) -> AR_W<FDCAN_TXBAR_SPEC, 0> {
        AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Tx Buffer Add Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBAR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbar::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbar::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBAR to value 0"]
impl crate::Resettable for FDCAN_TXBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
