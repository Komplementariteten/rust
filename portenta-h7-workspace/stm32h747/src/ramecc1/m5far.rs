#[doc = "Register `M5FAR` reader"]
pub type R = crate::R<M5FAR_SPEC>;
#[doc = "Register `M5FAR` writer"]
pub type W = crate::W<M5FAR_SPEC>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32>;
#[doc = "Field `FEC` writer - Failing error code"]
pub type FEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<M5FAR_SPEC, 0> {
        FEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5FAR_SPEC;
impl crate::RegisterSpec for M5FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5far::R`](R) reader structure"]
impl crate::Readable for M5FAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m5far::W`](W) writer structure"]
impl crate::Writable for M5FAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M5FAR to value 0"]
impl crate::Resettable for M5FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
