#[doc = "Register `LTR2` reader"]
pub type R = crate::R<LTR2_SPEC>;
#[doc = "Register `LTR2` writer"]
pub type W = crate::W<LTR2_SPEC>;
#[doc = "Field `LTR2` reader - Analog watchdog 2 lower threshold"]
pub type LTR2_R = crate::FieldReader<u32>;
#[doc = "Field `LTR2` writer - Analog watchdog 2 lower threshold"]
pub type LTR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&self) -> LTR2_R {
        LTR2_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ltr2(&mut self) -> LTR2_W<LTR2_SPEC, 0> {
        LTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC watchdog lower threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTR2_SPEC;
impl crate::RegisterSpec for LTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr2::R`](R) reader structure"]
impl crate::Readable for LTR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ltr2::W`](W) writer structure"]
impl crate::Writable for LTR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTR2 to value 0"]
impl crate::Resettable for LTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
