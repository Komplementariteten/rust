#[doc = "Register `HTR2` reader"]
pub type R = crate::R<HTR2_SPEC>;
#[doc = "Register `HTR2` writer"]
pub type W = crate::W<HTR2_SPEC>;
#[doc = "Field `HTR2` reader - Analog watchdog 2 higher threshold"]
pub type HTR2_R = crate::FieldReader<u32>;
#[doc = "Field `HTR2` writer - Analog watchdog 2 higher threshold"]
pub type HTR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn htr2(&self) -> HTR2_R {
        HTR2_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn htr2(&mut self) -> HTR2_W<HTR2_SPEC, 0> {
        HTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC watchdog higher threshold register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTR2_SPEC;
impl crate::RegisterSpec for HTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htr2::R`](R) reader structure"]
impl crate::Readable for HTR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`htr2::W`](W) writer structure"]
impl crate::Writable for HTR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTR2 to value 0"]
impl crate::Resettable for HTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
