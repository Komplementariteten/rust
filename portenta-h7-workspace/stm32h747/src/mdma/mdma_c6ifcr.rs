#[doc = "Register `MDMA_C6IFCR` writer"]
pub type W = crate::W<MDMA_C6IFCR_SPEC>;
#[doc = "Field `CTEIF6` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type CTEIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCTCIF6` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type CCTCIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBRTIF6` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type CBRTIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBTIF6` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type CBTIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLTCIF6` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type CLTCIF6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<MDMA_C6IFCR_SPEC, 0> {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif6(&mut self) -> CCTCIF6_W<MDMA_C6IFCR_SPEC, 1> {
        CCTCIF6_W::new(self)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbrtif6(&mut self) -> CBRTIF6_W<MDMA_C6IFCR_SPEC, 2> {
        CBRTIF6_W::new(self)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbtif6(&mut self) -> CBTIF6_W<MDMA_C6IFCR_SPEC, 3> {
        CBTIF6_W::new(self)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cltcif6(&mut self) -> CLTCIF6_W<MDMA_C6IFCR_SPEC, 4> {
        CLTCIF6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c6ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C6IFCR_SPEC;
impl crate::RegisterSpec for MDMA_C6IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c6ifcr::W`](W) writer structure"]
impl crate::Writable for MDMA_C6IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C6IFCR to value 0"]
impl crate::Resettable for MDMA_C6IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
