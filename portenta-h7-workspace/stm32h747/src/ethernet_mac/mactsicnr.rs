#[doc = "Register `MACTSICNR` reader"]
pub type R = crate::R<MACTSICNR_SPEC>;
#[doc = "Register `MACTSICNR` writer"]
pub type W = crate::W<MACTSICNR_SPEC>;
#[doc = "Field `TSIC` reader - TSIC"]
pub type TSIC_R = crate::FieldReader<u32>;
#[doc = "Field `TSIC` writer - TSIC"]
pub type TSIC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - TSIC"]
    #[inline(always)]
    pub fn tsic(&self) -> TSIC_R {
        TSIC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSIC"]
    #[inline(always)]
    #[must_use]
    pub fn tsic(&mut self) -> TSIC_W<MACTSICNR_SPEC, 0> {
        TSIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp Ingress correction nanosecond register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsicnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsicnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSICNR_SPEC;
impl crate::RegisterSpec for MACTSICNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsicnr::R`](R) reader structure"]
impl crate::Readable for MACTSICNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mactsicnr::W`](W) writer structure"]
impl crate::Writable for MACTSICNR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSICNR to value 0"]
impl crate::Resettable for MACTSICNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
