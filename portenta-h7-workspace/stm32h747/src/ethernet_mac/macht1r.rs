#[doc = "Register `MACHT1R` reader"]
pub type R = crate::R<MACHT1R_SPEC>;
#[doc = "Register `MACHT1R` writer"]
pub type W = crate::W<MACHT1R_SPEC>;
#[doc = "Field `HT63T32` reader - HT63T32"]
pub type HT63T32_R = crate::FieldReader<u32>;
#[doc = "Field `HT63T32` writer - HT63T32"]
pub type HT63T32_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - HT63T32"]
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT63T32"]
    #[inline(always)]
    #[must_use]
    pub fn ht63t32(&mut self) -> HT63T32_W<MACHT1R_SPEC, 0> {
        HT63T32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hash Table 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macht1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macht1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHT1R_SPEC;
impl crate::RegisterSpec for MACHT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macht1r::R`](R) reader structure"]
impl crate::Readable for MACHT1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macht1r::W`](W) writer structure"]
impl crate::Writable for MACHT1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHT1R to value 0"]
impl crate::Resettable for MACHT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
