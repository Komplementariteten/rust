#[doc = "Register `SAI_GCR` reader"]
pub type R = crate::R<SAI_GCR_SPEC>;
#[doc = "Register `SAI_GCR` writer"]
pub type W = crate::W<SAI_GCR_SPEC>;
#[doc = "Field `SYNCIN` reader - Synchronization inputs"]
pub type SYNCIN_R = crate::FieldReader;
#[doc = "Field `SYNCIN` writer - Synchronization inputs"]
pub type SYNCIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SYNCOUT` reader - Synchronization outputs These bits are set and cleared by software."]
pub type SYNCOUT_R = crate::FieldReader;
#[doc = "Field `SYNCOUT` writer - Synchronization outputs These bits are set and cleared by software."]
pub type SYNCOUT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization outputs These bits are set and cleared by software."]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<SAI_GCR_SPEC, 0> {
        SYNCIN_W::new(self)
    }
    #[doc = "Bits 4:5 - Synchronization outputs These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<SAI_GCR_SPEC, 4> {
        SYNCOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_GCR_SPEC;
impl crate::RegisterSpec for SAI_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_gcr::R`](R) reader structure"]
impl crate::Readable for SAI_GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sai_gcr::W`](W) writer structure"]
impl crate::Writable for SAI_GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_GCR to value 0"]
impl crate::Resettable for SAI_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
