#[doc = "Register `CALFACT2` reader"]
pub type R = crate::R<CALFACT2_SPEC>;
#[doc = "Register `CALFACT2` writer"]
pub type W = crate::W<CALFACT2_SPEC>;
#[doc = "Field `LINCALFACT` reader - Linearity Calibration Factor"]
pub type LINCALFACT_R = crate::FieldReader<u32>;
#[doc = "Field `LINCALFACT` writer - Linearity Calibration Factor"]
pub type LINCALFACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl R {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    #[must_use]
    pub fn lincalfact(&mut self) -> LINCALFACT_W<CALFACT2_SPEC, 0> {
        LINCALFACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC Calibration Factor register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALFACT2_SPEC;
impl crate::RegisterSpec for CALFACT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact2::R`](R) reader structure"]
impl crate::Readable for CALFACT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`calfact2::W`](W) writer structure"]
impl crate::Writable for CALFACT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALFACT2 to value 0"]
impl crate::Resettable for CALFACT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
