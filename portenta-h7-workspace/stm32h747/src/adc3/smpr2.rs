#[doc = "Register `SMPR2` reader"]
pub type R = crate::R<SMPR2_SPEC>;
#[doc = "Register `SMPR2` writer"]
pub type W = crate::W<SMPR2_SPEC>;
#[doc = "Field `SMP10` reader - ADC channel 10 sampling time selection"]
pub type SMP10_R = crate::FieldReader;
#[doc = "Field `SMP10` writer - ADC channel 10 sampling time selection"]
pub type SMP10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP11` reader - ADC channel 11 sampling time selection"]
pub type SMP11_R = crate::FieldReader;
#[doc = "Field `SMP11` writer - ADC channel 11 sampling time selection"]
pub type SMP11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP12` reader - ADC channel 12 sampling time selection"]
pub type SMP12_R = crate::FieldReader;
#[doc = "Field `SMP12` writer - ADC channel 12 sampling time selection"]
pub type SMP12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP13` reader - ADC channel 13 sampling time selection"]
pub type SMP13_R = crate::FieldReader;
#[doc = "Field `SMP13` writer - ADC channel 13 sampling time selection"]
pub type SMP13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP14` reader - ADC channel 14 sampling time selection"]
pub type SMP14_R = crate::FieldReader;
#[doc = "Field `SMP14` writer - ADC channel 14 sampling time selection"]
pub type SMP14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP15` reader - ADC channel 15 sampling time selection"]
pub type SMP15_R = crate::FieldReader;
#[doc = "Field `SMP15` writer - ADC channel 15 sampling time selection"]
pub type SMP15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP16` reader - ADC channel 16 sampling time selection"]
pub type SMP16_R = crate::FieldReader;
#[doc = "Field `SMP16` writer - ADC channel 16 sampling time selection"]
pub type SMP16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP17` reader - ADC channel 17 sampling time selection"]
pub type SMP17_R = crate::FieldReader;
#[doc = "Field `SMP17` writer - ADC channel 17 sampling time selection"]
pub type SMP17_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP18` reader - ADC channel 18 sampling time selection"]
pub type SMP18_R = crate::FieldReader;
#[doc = "Field `SMP18` writer - ADC channel 18 sampling time selection"]
pub type SMP18_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SMP19` reader - ADC channel 18 sampling time selection"]
pub type SMP19_R = crate::FieldReader;
#[doc = "Field `SMP19` writer - ADC channel 18 sampling time selection"]
pub type SMP19_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - ADC channel 16 sampling time selection"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - ADC channel 17 sampling time selection"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    pub fn smp19(&self) -> SMP19_R {
        SMP19_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC channel 10 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<SMPR2_SPEC, 0> {
        SMP10_W::new(self)
    }
    #[doc = "Bits 3:5 - ADC channel 11 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<SMPR2_SPEC, 3> {
        SMP11_W::new(self)
    }
    #[doc = "Bits 6:8 - ADC channel 12 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<SMPR2_SPEC, 6> {
        SMP12_W::new(self)
    }
    #[doc = "Bits 9:11 - ADC channel 13 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<SMPR2_SPEC, 9> {
        SMP13_W::new(self)
    }
    #[doc = "Bits 12:14 - ADC channel 14 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<SMPR2_SPEC, 12> {
        SMP14_W::new(self)
    }
    #[doc = "Bits 15:17 - ADC channel 15 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<SMPR2_SPEC, 15> {
        SMP15_W::new(self)
    }
    #[doc = "Bits 18:20 - ADC channel 16 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<SMPR2_SPEC, 18> {
        SMP16_W::new(self)
    }
    #[doc = "Bits 21:23 - ADC channel 17 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<SMPR2_SPEC, 21> {
        SMP17_W::new(self)
    }
    #[doc = "Bits 24:26 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<SMPR2_SPEC, 24> {
        SMP18_W::new(self)
    }
    #[doc = "Bits 27:29 - ADC channel 18 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp19(&mut self) -> SMP19_W<SMPR2_SPEC, 27> {
        SMP19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC sampling time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr2::R`](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smpr2::W`](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
