#[doc = "Register `DIFSEL` reader"]
pub type R = crate::R<DIFSEL_SPEC>;
#[doc = "Register `DIFSEL` writer"]
pub type W = crate::W<DIFSEL_SPEC>;
#[doc = "Field `DIFSEL` reader - ADC channel differential or single-ended mode for channel"]
pub type DIFSEL_R = crate::FieldReader<u32>;
#[doc = "Field `DIFSEL` writer - ADC channel differential or single-ended mode for channel"]
pub type DIFSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - ADC channel differential or single-ended mode for channel"]
    #[inline(always)]
    #[must_use]
    pub fn difsel(&mut self) -> DIFSEL_W<DIFSEL_SPEC, 0> {
        DIFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC channel differential or single-ended mode selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIFSEL_SPEC;
impl crate::RegisterSpec for DIFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difsel::R`](R) reader structure"]
impl crate::Readable for DIFSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`difsel::W`](W) writer structure"]
impl crate::Writable for DIFSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DIFSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
