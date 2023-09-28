#[doc = "Register `ESCR` reader"]
pub type R = crate::R<ESCR_SPEC>;
#[doc = "Register `ESCR` writer"]
pub type W = crate::W<ESCR_SPEC>;
#[doc = "Field `FSC` reader - Frame start delimiter code"]
pub type FSC_R = crate::FieldReader;
#[doc = "Field `FSC` writer - Frame start delimiter code"]
pub type FSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LSC` reader - Line start delimiter code"]
pub type LSC_R = crate::FieldReader;
#[doc = "Field `LSC` writer - Line start delimiter code"]
pub type LSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LEC` reader - Line end delimiter code"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - Line end delimiter code"]
pub type LEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `FEC` reader - Frame end delimiter code"]
pub type FEC_R = crate::FieldReader;
#[doc = "Field `FEC` writer - Frame end delimiter code"]
pub type FEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame start delimiter code"]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter code"]
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Frame end delimiter code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame start delimiter code"]
    #[inline(always)]
    #[must_use]
    pub fn fsc(&mut self) -> FSC_W<ESCR_SPEC, 0> {
        FSC_W::new(self)
    }
    #[doc = "Bits 8:15 - Line start delimiter code"]
    #[inline(always)]
    #[must_use]
    pub fn lsc(&mut self) -> LSC_W<ESCR_SPEC, 8> {
        LSC_W::new(self)
    }
    #[doc = "Bits 16:23 - Line end delimiter code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<ESCR_SPEC, 16> {
        LEC_W::new(self)
    }
    #[doc = "Bits 24:31 - Frame end delimiter code"]
    #[inline(always)]
    #[must_use]
    pub fn fec(&mut self) -> FEC_W<ESCR_SPEC, 24> {
        FEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "embedded synchronization code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESCR_SPEC;
impl crate::RegisterSpec for ESCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`escr::R`](R) reader structure"]
impl crate::Readable for ESCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`escr::W`](W) writer structure"]
impl crate::Writable for ESCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESCR to value 0"]
impl crate::Resettable for ESCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
