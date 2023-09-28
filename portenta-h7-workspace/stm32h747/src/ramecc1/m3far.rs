#[doc = "Register `M3FAR` reader"]
pub type R = crate::R<M3FAR_SPEC>;
#[doc = "Register `M3FAR` writer"]
pub type W = crate::W<M3FAR_SPEC>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
#[doc = "Field `FADD` writer - ECC error failing address"]
pub type FADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    #[must_use]
    pub fn fadd(&mut self) -> FADD_W<M3FAR_SPEC, 0> {
        FADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3far::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m3far::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3FAR_SPEC;
impl crate::RegisterSpec for M3FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3far::R`](R) reader structure"]
impl crate::Readable for M3FAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m3far::W`](W) writer structure"]
impl crate::Writable for M3FAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M3FAR to value 0"]
impl crate::Resettable for M3FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
