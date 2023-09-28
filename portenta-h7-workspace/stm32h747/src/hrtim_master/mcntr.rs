#[doc = "Register `MCNTR` reader"]
pub type R = crate::R<MCNTR_SPEC>;
#[doc = "Register `MCNTR` writer"]
pub type W = crate::W<MCNTR_SPEC>;
#[doc = "Field `MCNT` reader - Counter value"]
pub type MCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MCNT` writer - Counter value"]
pub type MCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<MCNTR_SPEC, 0> {
        MCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Timer Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCNTR_SPEC;
impl crate::RegisterSpec for MCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcntr::R`](R) reader structure"]
impl crate::Readable for MCNTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcntr::W`](W) writer structure"]
impl crate::Writable for MCNTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCNTR to value 0"]
impl crate::Resettable for MCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
