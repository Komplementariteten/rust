#[doc = "Register `DFSDM_FLT0FCR` reader"]
pub type R = crate::R<DFSDM_FLT0FCR_SPEC>;
#[doc = "Register `DFSDM_FLT0FCR` writer"]
pub type W = crate::W<DFSDM_FLT0FCR_SPEC>;
#[doc = "Field `IOSR` reader - Integrator oversampling ratio (averaging length)"]
pub type IOSR_R = crate::FieldReader;
#[doc = "Field `IOSR` writer - Integrator oversampling ratio (averaging length)"]
pub type IOSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)"]
pub type FOSR_R = crate::FieldReader<u16>;
#[doc = "Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)"]
pub type FOSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `FORD` reader - Sinc filter order"]
pub type FORD_R = crate::FieldReader;
#[doc = "Field `FORD` writer - Sinc filter order"]
pub type FORD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    #[must_use]
    pub fn iosr(&mut self) -> IOSR_W<DFSDM_FLT0FCR_SPEC, 0> {
        IOSR_W::new(self)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<DFSDM_FLT0FCR_SPEC, 16> {
        FOSR_W::new(self)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<DFSDM_FLT0FCR_SPEC, 29> {
        FORD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0FCR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0fcr::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0fcr::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT0FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT0FCR to value 0"]
impl crate::Resettable for DFSDM_FLT0FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
