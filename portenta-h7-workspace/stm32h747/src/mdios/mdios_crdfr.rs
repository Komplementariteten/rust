#[doc = "Register `MDIOS_CRDFR` reader"]
pub type R = crate::R<MDIOS_CRDFR_SPEC>;
#[doc = "Register `MDIOS_CRDFR` writer"]
pub type W = crate::W<MDIOS_CRDFR_SPEC>;
#[doc = "Field `CRDF` reader - Clear the read flag"]
pub type CRDF_R = crate::FieldReader<u32>;
#[doc = "Field `CRDF` writer - Clear the read flag"]
pub type CRDF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear the read flag"]
    #[inline(always)]
    pub fn crdf(&self) -> CRDF_R {
        CRDF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the read flag"]
    #[inline(always)]
    #[must_use]
    pub fn crdf(&mut self) -> CRDF_W<MDIOS_CRDFR_SPEC, 0> {
        CRDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS clear read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_crdfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_crdfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CRDFR_SPEC;
impl crate::RegisterSpec for MDIOS_CRDFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_crdfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CRDFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_crdfr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CRDFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_CRDFR to value 0"]
impl crate::Resettable for MDIOS_CRDFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
