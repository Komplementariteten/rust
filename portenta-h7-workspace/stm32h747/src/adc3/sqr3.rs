#[doc = "Register `SQR3` reader"]
pub type R = crate::R<SQR3_SPEC>;
#[doc = "Register `SQR3` writer"]
pub type W = crate::W<SQR3_SPEC>;
#[doc = "Field `SQ10` reader - ADC group regular sequencer rank 10"]
pub type SQ10_R = crate::FieldReader;
#[doc = "Field `SQ10` writer - ADC group regular sequencer rank 10"]
pub type SQ10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ11` reader - ADC group regular sequencer rank 11"]
pub type SQ11_R = crate::FieldReader;
#[doc = "Field `SQ11` writer - ADC group regular sequencer rank 11"]
pub type SQ11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ12` reader - ADC group regular sequencer rank 12"]
pub type SQ12_R = crate::FieldReader;
#[doc = "Field `SQ12` writer - ADC group regular sequencer rank 12"]
pub type SQ12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ13` reader - ADC group regular sequencer rank 13"]
pub type SQ13_R = crate::FieldReader;
#[doc = "Field `SQ13` writer - ADC group regular sequencer rank 13"]
pub type SQ13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SQ14` reader - ADC group regular sequencer rank 14"]
pub type SQ14_R = crate::FieldReader;
#[doc = "Field `SQ14` writer - ADC group regular sequencer rank 14"]
pub type SQ14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADC group regular sequencer rank 10"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 11"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC group regular sequencer rank 12"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC group regular sequencer rank 13"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADC group regular sequencer rank 14"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADC group regular sequencer rank 10"]
    #[inline(always)]
    #[must_use]
    pub fn sq10(&mut self) -> SQ10_W<SQR3_SPEC, 0> {
        SQ10_W::new(self)
    }
    #[doc = "Bits 6:10 - ADC group regular sequencer rank 11"]
    #[inline(always)]
    #[must_use]
    pub fn sq11(&mut self) -> SQ11_W<SQR3_SPEC, 6> {
        SQ11_W::new(self)
    }
    #[doc = "Bits 12:16 - ADC group regular sequencer rank 12"]
    #[inline(always)]
    #[must_use]
    pub fn sq12(&mut self) -> SQ12_W<SQR3_SPEC, 12> {
        SQ12_W::new(self)
    }
    #[doc = "Bits 18:22 - ADC group regular sequencer rank 13"]
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<SQR3_SPEC, 18> {
        SQ13_W::new(self)
    }
    #[doc = "Bits 24:28 - ADC group regular sequencer rank 14"]
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<SQR3_SPEC, 24> {
        SQ14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC group regular sequencer ranks register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SQR3_SPEC;
impl crate::RegisterSpec for SQR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqr3::R`](R) reader structure"]
impl crate::Readable for SQR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sqr3::W`](W) writer structure"]
impl crate::Writable for SQR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for SQR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
