#[doc = "Register `D1CCIPR` reader"]
pub type R = crate::R<D1CCIPR_SPEC>;
#[doc = "Register `D1CCIPR` writer"]
pub type W = crate::W<D1CCIPR_SPEC>;
#[doc = "Field `FMCSRC` reader - FMC kernel clock source selection"]
pub type FMCSRC_R = crate::FieldReader;
#[doc = "Field `FMCSRC` writer - FMC kernel clock source selection"]
pub type FMCSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `QSPISRC` reader - QUADSPI kernel clock source selection"]
pub type QSPISRC_R = crate::FieldReader;
#[doc = "Field `QSPISRC` writer - QUADSPI kernel clock source selection"]
pub type QSPISRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SDMMCSRC` reader - SDMMC kernel clock source selection"]
pub type SDMMCSRC_R = crate::BitReader;
#[doc = "Field `SDMMCSRC` writer - SDMMC kernel clock source selection"]
pub type SDMMCSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKPERSRC` reader - per_ck clock source selection"]
pub type CKPERSRC_R = crate::FieldReader;
#[doc = "Field `CKPERSRC` writer - per_ck clock source selection"]
pub type CKPERSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsrc(&self) -> FMCSRC_R {
        FMCSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsrc(&self) -> SDMMCSRC_R {
        SDMMCSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fmcsrc(&mut self) -> FMCSRC_W<D1CCIPR_SPEC, 0> {
        FMCSRC_W::new(self)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn qspisrc(&mut self) -> QSPISRC_W<D1CCIPR_SPEC, 4> {
        QSPISRC_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsrc(&mut self) -> SDMMCSRC_W<D1CCIPR_SPEC, 16> {
        SDMMCSRC_W::new(self)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W<D1CCIPR_SPEC, 28> {
        CKPERSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 1 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1CCIPR_SPEC;
impl crate::RegisterSpec for D1CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1ccipr::R`](R) reader structure"]
impl crate::Readable for D1CCIPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d1ccipr::W`](W) writer structure"]
impl crate::Writable for D1CCIPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D1CCIPR to value 0"]
impl crate::Resettable for D1CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
