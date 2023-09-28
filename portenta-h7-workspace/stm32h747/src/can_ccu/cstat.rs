#[doc = "Register `CSTAT` reader"]
pub type R = crate::R<CSTAT_SPEC>;
#[doc = "Register `CSTAT` writer"]
pub type W = crate::W<CSTAT_SPEC>;
#[doc = "Field `OCPC` reader - Oscillator Clock Period Counter"]
pub type OCPC_R = crate::FieldReader<u32>;
#[doc = "Field `OCPC` writer - Oscillator Clock Period Counter"]
pub type OCPC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
#[doc = "Field `TQC` reader - Time Quanta Counter"]
pub type TQC_R = crate::FieldReader<u16>;
#[doc = "Field `TQC` writer - Time Quanta Counter"]
pub type TQC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `CALS` reader - Calibration State"]
pub type CALS_R = crate::FieldReader;
#[doc = "Field `CALS` writer - Calibration State"]
pub type CALS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&self) -> OCPC_R {
        OCPC_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&self) -> TQC_R {
        TQC_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&self) -> CALS_R {
        CALS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ocpc(&mut self) -> OCPC_W<CSTAT_SPEC, 0> {
        OCPC_W::new(self)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tqc(&mut self) -> TQC_W<CSTAT_SPEC, 18> {
        TQC_W::new(self)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    #[must_use]
    pub fn cals(&mut self) -> CALS_W<CSTAT_SPEC, 30> {
        CALS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Calibration Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSTAT_SPEC;
impl crate::RegisterSpec for CSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstat::R`](R) reader structure"]
impl crate::Readable for CSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cstat::W`](W) writer structure"]
impl crate::Writable for CSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSTAT to value 0"]
impl crate::Resettable for CSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
