#[doc = "Register `M2FECR` reader"]
pub type R = crate::R<M2FECR_SPEC>;
#[doc = "Register `M2FECR` writer"]
pub type W = crate::W<M2FECR_SPEC>;
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
    pub fn fec(&mut self) -> FEC_W<M2FECR_SPEC, 0> {
        FEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m2fecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2FECR_SPEC;
impl crate::RegisterSpec for M2FECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2fecr::R`](R) reader structure"]
impl crate::Readable for M2FECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m2fecr::W`](W) writer structure"]
impl crate::Writable for M2FECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M2FECR to value 0"]
impl crate::Resettable for M2FECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
