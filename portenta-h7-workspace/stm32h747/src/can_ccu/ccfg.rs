#[doc = "Register `CCFG` reader"]
pub type R = crate::R<CCFG_SPEC>;
#[doc = "Register `CCFG` writer"]
pub type W = crate::W<CCFG_SPEC>;
#[doc = "Field `TQBT` reader - Time Quanta per Bit Time"]
pub type TQBT_R = crate::FieldReader;
#[doc = "Field `TQBT` writer - Time Quanta per Bit Time"]
pub type TQBT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `BCC` reader - Bypass Clock Calibration"]
pub type BCC_R = crate::BitReader;
#[doc = "Field `BCC` writer - Bypass Clock Calibration"]
pub type BCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFL` reader - Calibration Field Length"]
pub type CFL_R = crate::BitReader;
#[doc = "Field `CFL` writer - Calibration Field Length"]
pub type CFL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCPM` reader - Oscillator Clock Periods Minimum"]
pub type OCPM_R = crate::FieldReader;
#[doc = "Field `OCPM` writer - Oscillator Clock Periods Minimum"]
pub type OCPM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CDIV` reader - Clock Divider"]
pub type CDIV_R = crate::FieldReader;
#[doc = "Field `CDIV` writer - Clock Divider"]
pub type CDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    pub fn tqbt(&self) -> TQBT_R {
        TQBT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    pub fn cfl(&self) -> CFL_R {
        CFL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    pub fn cdiv(&self) -> CDIV_R {
        CDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    #[must_use]
    pub fn tqbt(&mut self) -> TQBT_W<CCFG_SPEC, 0> {
        TQBT_W::new(self)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn bcc(&mut self) -> BCC_W<CCFG_SPEC, 6> {
        BCC_W::new(self)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    #[must_use]
    pub fn cfl(&mut self) -> CFL_W<CCFG_SPEC, 7> {
        CFL_W::new(self)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    #[must_use]
    pub fn ocpm(&mut self) -> OCPM_W<CCFG_SPEC, 8> {
        OCPM_W::new(self)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn cdiv(&mut self) -> CDIV_W<CCFG_SPEC, 16> {
        CDIV_W::new(self)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<CCFG_SPEC, 31> {
        SWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Calibration Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCFG_SPEC;
impl crate::RegisterSpec for CCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg::R`](R) reader structure"]
impl crate::Readable for CCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccfg::W`](W) writer structure"]
impl crate::Writable for CCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG to value 0"]
impl crate::Resettable for CCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
