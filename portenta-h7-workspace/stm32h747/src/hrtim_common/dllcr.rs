#[doc = "Register `DLLCR` reader"]
pub type R = crate::R<DLLCR_SPEC>;
#[doc = "Register `DLLCR` writer"]
pub type W = crate::W<DLLCR_SPEC>;
#[doc = "Field `CAL` reader - DLL Calibration Start"]
pub type CAL_R = crate::BitReader;
#[doc = "Field `CAL` writer - DLL Calibration Start"]
pub type CAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALEN` reader - DLL Calibration Enable"]
pub type CALEN_R = crate::BitReader;
#[doc = "Field `CALEN` writer - DLL Calibration Enable"]
pub type CALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALRTE` reader - DLL Calibration rate"]
pub type CALRTE_R = crate::FieldReader;
#[doc = "Field `CALRTE` writer - DLL Calibration rate"]
pub type CALRTE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<DLLCR_SPEC, 0> {
        CAL_W::new(self)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<DLLCR_SPEC, 1> {
        CALEN_W::new(self)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    #[must_use]
    pub fn calrte(&mut self) -> CALRTE_W<DLLCR_SPEC, 2> {
        CALRTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DLL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dllcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dllcr::R`](R) reader structure"]
impl crate::Readable for DLLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dllcr::W`](W) writer structure"]
impl crate::Writable for DLLCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLLCR to value 0"]
impl crate::Resettable for DLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
