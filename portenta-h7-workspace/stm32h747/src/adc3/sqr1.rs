#[doc = "Register `SQR1` reader"]
pub type R = crate::R<SQR1_SPEC>;
#[doc = "Register `SQR1` writer"]
pub type W = crate::W<SQR1_SPEC>;
#[doc = "Field `L3` reader - L3"]
pub type L3_R = crate::FieldReader;
#[doc = "Field `L3` writer - L3"]
pub type L3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SQ1` reader - ADC group regular sequencer rank 1"]
pub type SQ1_R = crate::FieldReader;
#[doc = "Field `SQ1` writer - ADC group regular sequencer rank 1"]
pub type SQ1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ2` reader - ADC group regular sequencer rank 2"]
pub type SQ2_R = crate::FieldReader;
#[doc = "Field `SQ2` writer - ADC group regular sequencer rank 2"]
pub type SQ2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ3` reader - ADC group regular sequencer rank 3"]
pub type SQ3_R = crate::FieldReader;
#[doc = "Field `SQ3` writer - ADC group regular sequencer rank 3"]
pub type SQ3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ4` reader - ADC group regular sequencer rank 4"]
pub type SQ4_R = crate::FieldReader;
#[doc = "Field `SQ4` writer - ADC group regular sequencer rank 4"]
pub type SQ4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:3 - L3"]
    #[inline(always)]
    pub fn l3(&self) -> L3_R {
        L3_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC group regular sequencer rank 2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC group regular sequencer rank 3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC group regular sequencer rank 4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - L3"]
    #[inline(always)]
    #[must_use]
    pub fn l3(&mut self) -> L3_W<SQR1_SPEC, 0> {
        L3_W::new(self)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 1"]
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<SQR1_SPEC, 6> {
        SQ1_W::new(self)
    }
    #[doc = "Bits 12:16 - ADC group regular sequencer rank 2"]
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<SQR1_SPEC, 12> {
        SQ2_W::new(self)
    }
    #[doc = "Bits 18:22 - ADC group regular sequencer rank 3"]
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<SQR1_SPEC, 18> {
        SQ3_W::new(self)
    }
    #[doc = "Bits 24:28 - ADC group regular sequencer rank 4"]
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<SQR1_SPEC, 24> {
        SQ4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC group regular sequencer ranks register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr1::R`](R) reader structure"]
impl crate::Readable for SQR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr1::W`](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
