#[doc = "Register `LTR3` reader"]
pub type R = crate::R<LTR3_SPEC>;
#[doc = "Register `LTR3` writer"]
pub type W = crate::W<LTR3_SPEC>;
#[doc = "Field `LTR3` reader - Analog watchdog 3 lower threshold"]
pub type LTR3_R = crate::FieldReader<u32>;
#[doc = "Field `LTR3` writer - Analog watchdog 3 lower threshold"]
pub type LTR3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn ltr3(&self) -> LTR3_R {
        LTR3_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ltr3(&mut self) -> LTR3_W<LTR3_SPEC, 0> {
        LTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC watchdog lower threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTR3_SPEC;
impl crate::RegisterSpec for LTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr3::R`](R) reader structure"]
impl crate::Readable for LTR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ltr3::W`](W) writer structure"]
impl crate::Writable for LTR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTR3 to value 0"]
impl crate::Resettable for LTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
