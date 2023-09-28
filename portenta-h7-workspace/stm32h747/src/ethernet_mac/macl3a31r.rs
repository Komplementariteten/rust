#[doc = "Register `MACL3A31R` reader"]
pub type R = crate::R<MACL3A31R_SPEC>;
#[doc = "Register `MACL3A31R` writer"]
pub type W = crate::W<MACL3A31R_SPEC>;
#[doc = "Field `L3A31` reader - L3A31"]
pub type L3A31_R = crate::FieldReader<u32>;
#[doc = "Field `L3A31` writer - L3A31"]
pub type L3A31_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A31"]
    #[inline(always)]
    pub fn l3a31(&self) -> L3A31_R {
        L3A31_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A31"]
    #[inline(always)]
    #[must_use]
    pub fn l3a31(&mut self) -> L3A31_W<MACL3A31R_SPEC, 0> {
        L3A31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layer3 address 3 filter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a31r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a31r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3A31R_SPEC;
impl crate::RegisterSpec for MACL3A31R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a31r::R`](R) reader structure"]
impl crate::Readable for MACL3A31R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macl3a31r::W`](W) writer structure"]
impl crate::Writable for MACL3A31R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACL3A31R to value 0"]
impl crate::Resettable for MACL3A31R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
