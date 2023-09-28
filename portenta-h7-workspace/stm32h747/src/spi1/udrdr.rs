#[doc = "Register `UDRDR` reader"]
pub type R = crate::R<UDRDR_SPEC>;
#[doc = "Register `UDRDR` writer"]
pub type W = crate::W<UDRDR_SPEC>;
#[doc = "Field `UDRDR` reader - Data at slave underrun condition"]
pub type UDRDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDRDR` writer - Data at slave underrun condition"]
pub type UDRDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Data at slave underrun condition"]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data at slave underrun condition"]
    #[inline(always)]
    #[must_use]
    pub fn udrdr(&mut self) -> UDRDR_W<UDRDR_SPEC, 0> {
        UDRDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Underrun Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDRDR_SPEC;
impl crate::RegisterSpec for UDRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udrdr::R`](R) reader structure"]
impl crate::Readable for UDRDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udrdr::W`](W) writer structure"]
impl crate::Writable for UDRDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDRDR to value 0"]
impl crate::Resettable for UDRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
