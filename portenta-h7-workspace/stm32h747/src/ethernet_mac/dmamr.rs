#[doc = "Register `DMAMR` reader"]
pub type R = crate::R<DMAMR_SPEC>;
#[doc = "Register `DMAMR` writer"]
pub type W = crate::W<DMAMR_SPEC>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DA` reader - DMA Tx or Rx Arbitration Scheme"]
pub type DA_R = crate::BitReader;
#[doc = "Field `TXPR` reader - Transmit priority"]
pub type TXPR_R = crate::BitReader;
#[doc = "Field `PR` reader - Priority ratio"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `INTM` reader - Interrupt Mode"]
pub type INTM_R = crate::BitReader;
#[doc = "Field `INTM` writer - Interrupt Mode"]
pub type INTM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Interrupt Mode"]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<DMAMR_SPEC, 0> {
        SWR_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<DMAMR_SPEC, 16> {
        INTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAMR_SPEC;
impl crate::RegisterSpec for DMAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamr::R`](R) reader structure"]
impl crate::Readable for DMAMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmamr::W`](W) writer structure"]
impl crate::Writable for DMAMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMR to value 0"]
impl crate::Resettable for DMAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
