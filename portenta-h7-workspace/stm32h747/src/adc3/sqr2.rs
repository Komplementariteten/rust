#[doc = "Register `SQR2` reader"]
pub type R = crate::R<SQR2_SPEC>;
#[doc = "Register `SQR2` writer"]
pub type W = crate::W<SQR2_SPEC>;
#[doc = "Field `SQ5` reader - ADC group regular sequencer rank 5"]
pub type SQ5_R = crate::FieldReader;
#[doc = "Field `SQ5` writer - ADC group regular sequencer rank 5"]
pub type SQ5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ6` reader - ADC group regular sequencer rank 6"]
pub type SQ6_R = crate::FieldReader;
#[doc = "Field `SQ6` writer - ADC group regular sequencer rank 6"]
pub type SQ6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ7` reader - ADC group regular sequencer rank 7"]
pub type SQ7_R = crate::FieldReader;
#[doc = "Field `SQ7` writer - ADC group regular sequencer rank 7"]
pub type SQ7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ8` reader - ADC group regular sequencer rank 8"]
pub type SQ8_R = crate::FieldReader;
#[doc = "Field `SQ8` writer - ADC group regular sequencer rank 8"]
pub type SQ8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ9` reader - ADC group regular sequencer rank 9"]
pub type SQ9_R = crate::FieldReader;
#[doc = "Field `SQ9` writer - ADC group regular sequencer rank 9"]
pub type SQ9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADC group regular sequencer rank 5"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 6"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC group regular sequencer rank 7"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC group regular sequencer rank 8"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC group regular sequencer rank 9"]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC group regular sequencer rank 5"]
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<SQR2_SPEC, 0> {
        SQ5_W::new(self)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 6"]
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<SQR2_SPEC, 6> {
        SQ6_W::new(self)
    }
    #[doc = "Bits 12:16 - ADC group regular sequencer rank 7"]
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<SQR2_SPEC, 12> {
        SQ7_W::new(self)
    }
    #[doc = "Bits 18:22 - ADC group regular sequencer rank 8"]
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<SQR2_SPEC, 18> {
        SQ8_W::new(self)
    }
    #[doc = "Bits 24:28 - ADC group regular sequencer rank 9"]
    #[inline(always)]
    #[must_use]
    pub fn sq9(&mut self) -> SQ9_W<SQR2_SPEC, 24> {
        SQ9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC group regular sequencer ranks register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr2::R`](R) reader structure"]
impl crate::Readable for SQR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr2::W`](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
