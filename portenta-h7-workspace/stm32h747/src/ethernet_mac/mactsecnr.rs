#[doc = "Register `MACTSECNR` reader"]
pub type R = crate::R<MACTSECNR_SPEC>;
#[doc = "Register `MACTSECNR` writer"]
pub type W = crate::W<MACTSECNR_SPEC>;
#[doc = "Field `TSEC` reader - TSEC"]
pub type TSEC_R = crate::FieldReader<u32>;
#[doc = "Field `TSEC` writer - TSEC"]
pub type TSEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    #[must_use]
    pub fn tsec(&mut self) -> TSEC_W<MACTSECNR_SPEC, 0> {
        TSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp Egress correction nanosecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsecnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsecnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSECNR_SPEC;
impl crate::RegisterSpec for MACTSECNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsecnr::R`](R) reader structure"]
impl crate::Readable for MACTSECNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mactsecnr::W`](W) writer structure"]
impl crate::Writable for MACTSECNR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSECNR to value 0"]
impl crate::Resettable for MACTSECNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
