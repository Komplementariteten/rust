#[doc = "Register `SAI_PDMDLY` reader"]
pub type R = crate::R<SAI_PDMDLY_SPEC>;
#[doc = "Register `SAI_PDMDLY` writer"]
pub type W = crate::W<SAI_PDMDLY_SPEC>;
#[doc = "Field `DLYM1L` reader - Delay line adjust for first microphone of pair 1"]
pub type DLYM1L_R = crate::FieldReader;
#[doc = "Field `DLYM1L` writer - Delay line adjust for first microphone of pair 1"]
pub type DLYM1L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM1R` reader - Delay line adjust for second microphone of pair 1"]
pub type DLYM1R_R = crate::FieldReader;
#[doc = "Field `DLYM1R` writer - Delay line adjust for second microphone of pair 1"]
pub type DLYM1R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM2L` reader - Delay line for first microphone of pair 2"]
pub type DLYM2L_R = crate::FieldReader;
#[doc = "Field `DLYM2L` writer - Delay line for first microphone of pair 2"]
pub type DLYM2L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM2R` reader - Delay line for second microphone of pair 2"]
pub type DLYM2R_R = crate::FieldReader;
#[doc = "Field `DLYM2R` writer - Delay line for second microphone of pair 2"]
pub type DLYM2R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM3L` reader - Delay line for first microphone of pair 3"]
pub type DLYM3L_R = crate::FieldReader;
#[doc = "Field `DLYM3L` writer - Delay line for first microphone of pair 3"]
pub type DLYM3L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM3R` reader - Delay line for second microphone of pair 3"]
pub type DLYM3R_R = crate::FieldReader;
#[doc = "Field `DLYM3R` writer - Delay line for second microphone of pair 3"]
pub type DLYM3R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM4L` reader - Delay line for first microphone of pair 4"]
pub type DLYM4L_R = crate::FieldReader;
#[doc = "Field `DLYM4L` writer - Delay line for first microphone of pair 4"]
pub type DLYM4L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DLYM4R` reader - Delay line for second microphone of pair 4"]
pub type DLYM4R_R = crate::FieldReader;
#[doc = "Field `DLYM4R` writer - Delay line for second microphone of pair 4"]
pub type DLYM4R_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1l(&mut self) -> DLYM1L_W<SAI_PDMDLY_SPEC, 0> {
        DLYM1L_W::new(self)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    #[must_use]
    pub fn dlym1r(&mut self) -> DLYM1R_W<SAI_PDMDLY_SPEC, 4> {
        DLYM1R_W::new(self)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2l(&mut self) -> DLYM2L_W<SAI_PDMDLY_SPEC, 8> {
        DLYM2L_W::new(self)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline(always)]
    #[must_use]
    pub fn dlym2r(&mut self) -> DLYM2R_W<SAI_PDMDLY_SPEC, 12> {
        DLYM2R_W::new(self)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3l(&mut self) -> DLYM3L_W<SAI_PDMDLY_SPEC, 16> {
        DLYM3L_W::new(self)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline(always)]
    #[must_use]
    pub fn dlym3r(&mut self) -> DLYM3R_W<SAI_PDMDLY_SPEC, 20> {
        DLYM3R_W::new(self)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4l(&mut self) -> DLYM4L_W<SAI_PDMDLY_SPEC, 24> {
        DLYM4L_W::new(self)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline(always)]
    #[must_use]
    pub fn dlym4r(&mut self) -> DLYM4R_W<SAI_PDMDLY_SPEC, 28> {
        DLYM4R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_PDMDLY_SPEC;
impl crate::RegisterSpec for SAI_PDMDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_pdmdly::R`](R) reader structure"]
impl crate::Readable for SAI_PDMDLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sai_pdmdly::W`](W) writer structure"]
impl crate::Writable for SAI_PDMDLY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_PDMDLY to value 0"]
impl crate::Resettable for SAI_PDMDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
