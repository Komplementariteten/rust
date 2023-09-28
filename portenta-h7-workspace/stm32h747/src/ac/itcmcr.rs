#[doc = "Register `ITCMCR` reader"]
pub type R = crate::R<ITCMCR_SPEC>;
#[doc = "Register `ITCMCR` writer"]
pub type W = crate::W<ITCMCR_SPEC>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RMW` reader - RMW"]
pub type RMW_R = crate::BitReader;
#[doc = "Field `RMW` writer - RMW"]
pub type RMW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RETEN` reader - RETEN"]
pub type RETEN_R = crate::BitReader;
#[doc = "Field `RETEN` writer - RETEN"]
pub type RETEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SZ` reader - SZ"]
pub type SZ_R = crate::FieldReader;
#[doc = "Field `SZ` writer - SZ"]
pub type SZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RMW"]
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RETEN"]
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - SZ"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<ITCMCR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - RMW"]
    #[inline(always)]
    #[must_use]
    pub fn rmw(&mut self) -> RMW_W<ITCMCR_SPEC, 1> {
        RMW_W::new(self)
    }
    #[doc = "Bit 2 - RETEN"]
    #[inline(always)]
    #[must_use]
    pub fn reten(&mut self) -> RETEN_W<ITCMCR_SPEC, 2> {
        RETEN_W::new(self)
    }
    #[doc = "Bits 3:6 - SZ"]
    #[inline(always)]
    #[must_use]
    pub fn sz(&mut self) -> SZ_W<ITCMCR_SPEC, 3> {
        SZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itcmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itcmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITCMCR_SPEC;
impl crate::RegisterSpec for ITCMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itcmcr::R`](R) reader structure"]
impl crate::Readable for ITCMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itcmcr::W`](W) writer structure"]
impl crate::Writable for ITCMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ITCMCR to value 0"]
impl crate::Resettable for ITCMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
