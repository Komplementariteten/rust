#[doc = "Register `MDMA_C15IFCR` writer"]
pub type W = crate::W<MDMA_C15IFCR_SPEC>;
#[doc = "Field `CTEIF15` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type CTEIF15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCTCIF15` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type CCTCIF15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBRTIF15` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type CBRTIF15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBTIF15` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type CBTIF15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLTCIF15` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type CLTCIF15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cteif15(&mut self) -> CTEIF15_W<MDMA_C15IFCR_SPEC, 0> {
        CTEIF15_W::new(self)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif15(&mut self) -> CCTCIF15_W<MDMA_C15IFCR_SPEC, 1> {
        CCTCIF15_W::new(self)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbrtif15(&mut self) -> CBRTIF15_W<MDMA_C15IFCR_SPEC, 2> {
        CBRTIF15_W::new(self)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbtif15(&mut self) -> CBTIF15_W<MDMA_C15IFCR_SPEC, 3> {
        CBTIF15_W::new(self)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cltcif15(&mut self) -> CLTCIF15_W<MDMA_C15IFCR_SPEC, 4> {
        CLTCIF15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c15ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C15IFCR_SPEC;
impl crate::RegisterSpec for MDMA_C15IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c15ifcr::W`](W) writer structure"]
impl crate::Writable for MDMA_C15IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C15IFCR to value 0"]
impl crate::Resettable for MDMA_C15IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
