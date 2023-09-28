#[doc = "Register `FDCAN_TXBTO` reader"]
pub type R = crate::R<FDCAN_TXBTO_SPEC>;
#[doc = "Register `FDCAN_TXBTO` writer"]
pub type W = crate::W<FDCAN_TXBTO_SPEC>;
#[doc = "Field `TO` reader - Transmission Occurred."]
pub type TO_R = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Transmission Occurred."]
pub type TO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmission Occurred."]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<FDCAN_TXBTO_SPEC, 0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_txbto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBTO_SPEC;
impl crate::RegisterSpec for FDCAN_TXBTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbto::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txbto::W`](W) writer structure"]
impl crate::Writable for FDCAN_TXBTO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBTO to value 0"]
impl crate::Resettable for FDCAN_TXBTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
