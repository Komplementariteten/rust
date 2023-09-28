#[doc = "Register `MDIOS_CWRFR` reader"]
pub type R = crate::R<MDIOS_CWRFR_SPEC>;
#[doc = "Register `MDIOS_CWRFR` writer"]
pub type W = crate::W<MDIOS_CWRFR_SPEC>;
#[doc = "Field `CWRF` reader - Clear the write flag"]
pub type CWRF_R = crate::FieldReader<u32>;
#[doc = "Field `CWRF` writer - Clear the write flag"]
pub type CWRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwrf(&mut self) -> CWRF_W<MDIOS_CWRFR_SPEC, 0> {
        CWRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS clear write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_cwrfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_cwrfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CWRFR_SPEC;
impl crate::RegisterSpec for MDIOS_CWRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_cwrfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CWRFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_cwrfr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CWRFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_CWRFR to value 0"]
impl crate::Resettable for MDIOS_CWRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
