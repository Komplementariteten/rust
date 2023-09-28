#[doc = "Register `MACHT0R` reader"]
pub type R = crate::R<MACHT0R_SPEC>;
#[doc = "Register `MACHT0R` writer"]
pub type W = crate::W<MACHT0R_SPEC>;
#[doc = "Field `HT31T0` reader - HT31T0"]
pub type HT31T0_R = crate::FieldReader<u32>;
#[doc = "Field `HT31T0` writer - HT31T0"]
pub type HT31T0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    #[must_use]
    pub fn ht31t0(&mut self) -> HT31T0_W<MACHT0R_SPEC, 0> {
        HT31T0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hash Table 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macht0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macht0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHT0R_SPEC;
impl crate::RegisterSpec for MACHT0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macht0r::R`](R) reader structure"]
impl crate::Readable for MACHT0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macht0r::W`](W) writer structure"]
impl crate::Writable for MACHT0R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHT0R to value 0"]
impl crate::Resettable for MACHT0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
