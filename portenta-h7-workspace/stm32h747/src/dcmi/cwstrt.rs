#[doc = "Register `CWSTRT` reader"]
pub type R = crate::R<CWSTRT_SPEC>;
#[doc = "Register `CWSTRT` writer"]
pub type W = crate::W<CWSTRT_SPEC>;
#[doc = "Field `HOFFCNT` reader - Horizontal offset count"]
pub type HOFFCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HOFFCNT` writer - Horizontal offset count"]
pub type HOFFCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `VST` reader - Vertical start line count"]
pub type VST_R = crate::FieldReader<u16>;
#[doc = "Field `VST` writer - Vertical start line count"]
pub type VST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Horizontal offset count"]
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Vertical start line count"]
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Horizontal offset count"]
    #[inline(always)]
    #[must_use]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W<CWSTRT_SPEC, 0> {
        HOFFCNT_W::new(self)
    }
    #[doc = "Bits 16:28 - Vertical start line count"]
    #[inline(always)]
    #[must_use]
    pub fn vst(&mut self) -> VST_W<CWSTRT_SPEC, 16> {
        VST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwstrt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwstrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSTRT_SPEC;
impl crate::RegisterSpec for CWSTRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwstrt::R`](R) reader structure"]
impl crate::Readable for CWSTRT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwstrt::W`](W) writer structure"]
impl crate::Writable for CWSTRT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWSTRT to value 0"]
impl crate::Resettable for CWSTRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
