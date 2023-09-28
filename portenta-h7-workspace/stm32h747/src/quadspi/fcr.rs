#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCR_SPEC>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `CTEF` reader - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
pub type CTEF_R = crate::BitReader;
#[doc = "Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
pub type CTEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCF` reader - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
pub type CTCF_R = crate::BitReader;
#[doc = "Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
pub type CTCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSMF` reader - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
pub type CSMF_R = crate::BitReader;
#[doc = "Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
pub type CSMF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTOF` reader - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
pub type CTOF_R = crate::BitReader;
#[doc = "Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
pub type CTOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctef(&self) -> CTEF_R {
        CTEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctcf(&self) -> CTCF_R {
        CTCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn csmf(&self) -> CSMF_R {
        CSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctof(&self) -> CTOF_R {
        CTOF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<FCR_SPEC, 0> {
        CTEF_W::new(self)
    }
    #[doc = "Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<FCR_SPEC, 1> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<FCR_SPEC, 3> {
        CSMF_W::new(self)
    }
    #[doc = "Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<FCR_SPEC, 4> {
        CTOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QUADSPI flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
