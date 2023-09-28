#[doc = "Register `MACECR` reader"]
pub type R = crate::R<MACECR_SPEC>;
#[doc = "Register `MACECR` writer"]
pub type W = crate::W<MACECR_SPEC>;
#[doc = "Field `GPSL` reader - GPSL"]
pub type GPSL_R = crate::FieldReader<u16>;
#[doc = "Field `GPSL` writer - GPSL"]
pub type GPSL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `DCRCC` reader - DCRCC"]
pub type DCRCC_R = crate::BitReader;
#[doc = "Field `DCRCC` writer - DCRCC"]
pub type DCRCC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPEN` reader - SPEN"]
pub type SPEN_R = crate::BitReader;
#[doc = "Field `SPEN` writer - SPEN"]
pub type SPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USP` reader - USP"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - USP"]
pub type USP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIPGEN` reader - EIPGEN"]
pub type EIPGEN_R = crate::BitReader;
#[doc = "Field `EIPGEN` writer - EIPGEN"]
pub type EIPGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIPG` reader - EIPG"]
pub type EIPG_R = crate::FieldReader;
#[doc = "Field `EIPG` writer - EIPG"]
pub type EIPG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - GPSL"]
    #[inline(always)]
    #[must_use]
    pub fn gpsl(&mut self) -> GPSL_W<MACECR_SPEC, 0> {
        GPSL_W::new(self)
    }
    #[doc = "Bit 16 - DCRCC"]
    #[inline(always)]
    #[must_use]
    pub fn dcrcc(&mut self) -> DCRCC_W<MACECR_SPEC, 16> {
        DCRCC_W::new(self)
    }
    #[doc = "Bit 17 - SPEN"]
    #[inline(always)]
    #[must_use]
    pub fn spen(&mut self) -> SPEN_W<MACECR_SPEC, 17> {
        SPEN_W::new(self)
    }
    #[doc = "Bit 18 - USP"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<MACECR_SPEC, 18> {
        USP_W::new(self)
    }
    #[doc = "Bit 24 - EIPGEN"]
    #[inline(always)]
    #[must_use]
    pub fn eipgen(&mut self) -> EIPGEN_W<MACECR_SPEC, 24> {
        EIPGEN_W::new(self)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EIPG_W<MACECR_SPEC, 25> {
        EIPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Extended operating mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACECR_SPEC;
impl crate::RegisterSpec for MACECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macecr::R`](R) reader structure"]
impl crate::Readable for MACECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macecr::W`](W) writer structure"]
impl crate::Writable for MACECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACECR to value 0"]
impl crate::Resettable for MACECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
