#[doc = "Register `CALFACT` reader"]
pub type R = crate::R<CALFACT_SPEC>;
#[doc = "Register `CALFACT` writer"]
pub type W = crate::W<CALFACT_SPEC>;
#[doc = "Field `CALFACT_S` reader - ADC calibration factor in single-ended mode"]
pub type CALFACT_S_R = crate::FieldReader<u16>;
#[doc = "Field `CALFACT_S` writer - ADC calibration factor in single-ended mode"]
pub type CALFACT_S_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `CALFACT_D` reader - ADC calibration factor in differential mode"]
pub type CALFACT_D_R = crate::FieldReader<u16>;
#[doc = "Field `CALFACT_D` writer - ADC calibration factor in differential mode"]
pub type CALFACT_D_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - ADC calibration factor in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<CALFACT_SPEC, 0> {
        CALFACT_S_W::new(self)
    }
    #[doc = "Bits 16:26 - ADC calibration factor in differential mode"]
    #[inline(always)]
    #[must_use]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<CALFACT_SPEC, 16> {
        CALFACT_D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALFACT_SPEC;
impl crate::RegisterSpec for CALFACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact::R`](R) reader structure"]
impl crate::Readable for CALFACT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calfact::W`](W) writer structure"]
impl crate::Writable for CALFACT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CALFACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
