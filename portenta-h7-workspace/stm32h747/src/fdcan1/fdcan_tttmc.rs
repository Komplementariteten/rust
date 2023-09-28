#[doc = "Register `FDCAN_TTTMC` reader"]
pub type R = crate::R<FDCAN_TTTMC_SPEC>;
#[doc = "Register `FDCAN_TTTMC` writer"]
pub type W = crate::W<FDCAN_TTTMC_SPEC>;
#[doc = "Field `TMSA` reader - Trigger Memory Start Address"]
pub type TMSA_R = crate::FieldReader<u16>;
#[doc = "Field `TMSA` writer - Trigger Memory Start Address"]
pub type TMSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `TME` reader - Trigger Memory Elements"]
pub type TME_R = crate::FieldReader;
#[doc = "Field `TME` writer - Trigger Memory Elements"]
pub type TME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn tmsa(&mut self) -> TMSA_W<FDCAN_TTTMC_SPEC, 2> {
        TMSA_W::new(self)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TME_W<FDCAN_TTTMC_SPEC, 16> {
        TME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTTMC_SPEC;
impl crate::RegisterSpec for FDCAN_TTTMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tttmc::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTTMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tttmc::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTTMC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTMC to value 0"]
impl crate::Resettable for FDCAN_TTTMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
