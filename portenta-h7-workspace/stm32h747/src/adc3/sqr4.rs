#[doc = "Register `SQR4` reader"]
pub type R = crate::R<SQR4_SPEC>;
#[doc = "Register `SQR4` writer"]
pub type W = crate::W<SQR4_SPEC>;
#[doc = "Field `SQ15` reader - ADC group regular sequencer rank 15"]
pub type SQ15_R = crate::FieldReader;
#[doc = "Field `SQ15` writer - ADC group regular sequencer rank 15"]
pub type SQ15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ16` reader - ADC group regular sequencer rank 16"]
pub type SQ16_R = crate::FieldReader;
#[doc = "Field `SQ16` writer - ADC group regular sequencer rank 16"]
pub type SQ16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADC group regular sequencer rank 15"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 16"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC group regular sequencer rank 15"]
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<SQR4_SPEC, 0> {
        SQ15_W::new(self)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 16"]
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<SQR4_SPEC, 6> {
        SQ16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC group regular sequencer ranks register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR4_SPEC;
impl crate::RegisterSpec for SQR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr4::R`](R) reader structure"]
impl crate::Readable for SQR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr4::W`](W) writer structure"]
impl crate::Writable for SQR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR4 to value 0"]
impl crate::Resettable for SQR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
