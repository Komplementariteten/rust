#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JSQR_SPEC>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JSQR_SPEC>;
#[doc = "Field `JL` reader - ADC group injected sequencer scan length"]
pub type JL_R = crate::FieldReader;
#[doc = "Field `JL` writer - ADC group injected sequencer scan length"]
pub type JL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `JEXTSEL` reader - ADC group injected external trigger source"]
pub type JEXTSEL_R = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - ADC group injected external trigger source"]
pub type JEXTSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JEXTEN` reader - ADC group injected external trigger polarity"]
pub type JEXTEN_R = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - ADC group injected external trigger polarity"]
pub type JEXTEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `JSQ1` reader - ADC group injected sequencer rank 1"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - ADC group injected sequencer rank 1"]
pub type JSQ1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JSQ2` reader - ADC group injected sequencer rank 2"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - ADC group injected sequencer rank 2"]
pub type JSQ2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JSQ3` reader - ADC group injected sequencer rank 3"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - ADC group injected sequencer rank 3"]
pub type JSQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `JSQ4` reader - ADC group injected sequencer rank 4"]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - ADC group injected sequencer rank 4"]
pub type JSQ4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:1 - ADC group injected sequencer scan length"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - ADC group injected external trigger source"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:8 - ADC group injected external trigger polarity"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:13 - ADC group injected sequencer rank 1"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - ADC group injected sequencer rank 2"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - ADC group injected sequencer rank 3"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - ADC group injected sequencer rank 4"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC group injected sequencer scan length"]
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<JSQR_SPEC, 0> {
        JL_W::new(self)
    }
    #[doc = "Bits 2:6 - ADC group injected external trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<JSQR_SPEC, 2> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bits 7:8 - ADC group injected external trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<JSQR_SPEC, 7> {
        JEXTEN_W::new(self)
    }
    #[doc = "Bits 9:13 - ADC group injected sequencer rank 1"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<JSQR_SPEC, 9> {
        JSQ1_W::new(self)
    }
    #[doc = "Bits 15:19 - ADC group injected sequencer rank 2"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<JSQR_SPEC, 15> {
        JSQ2_W::new(self)
    }
    #[doc = "Bits 21:25 - ADC group injected sequencer rank 3"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<JSQR_SPEC, 21> {
        JSQ3_W::new(self)
    }
    #[doc = "Bits 27:31 - ADC group injected sequencer rank 4"]
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<JSQR_SPEC, 27> {
        JSQ4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC group injected sequencer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JSQR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JSQR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
