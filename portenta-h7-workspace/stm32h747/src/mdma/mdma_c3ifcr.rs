#[doc = "Register `MDMA_C3IFCR` writer"]
pub type W = crate::W<MDMA_C3IFCR_SPEC>;
#[doc = "Field `CTEIF3` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type CTEIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCTCIF3` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type CCTCIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBRTIF3` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type CBRTIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBTIF3` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type CBTIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLTCIF3` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type CLTCIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<MDMA_C3IFCR_SPEC, 0> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif3(&mut self) -> CCTCIF3_W<MDMA_C3IFCR_SPEC, 1> {
        CCTCIF3_W::new(self)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbrtif3(&mut self) -> CBRTIF3_W<MDMA_C3IFCR_SPEC, 2> {
        CBRTIF3_W::new(self)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cbtif3(&mut self) -> CBTIF3_W<MDMA_C3IFCR_SPEC, 3> {
        CBTIF3_W::new(self)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    #[must_use]
    pub fn cltcif3(&mut self) -> CLTCIF3_W<MDMA_C3IFCR_SPEC, 4> {
        CLTCIF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c3ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C3IFCR_SPEC;
impl crate::RegisterSpec for MDMA_C3IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c3ifcr::W`](W) writer structure"]
impl crate::Writable for MDMA_C3IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C3IFCR to value 0"]
impl crate::Resettable for MDMA_C3IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
