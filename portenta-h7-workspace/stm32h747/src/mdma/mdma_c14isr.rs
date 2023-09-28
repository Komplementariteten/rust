#[doc = "Register `MDMA_C14ISR` reader"]
pub type R = crate::R<MDMA_C14ISR_SPEC>;
#[doc = "Field `TEIF14` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type TEIF14_R = crate::BitReader;
#[doc = "Field `CTCIF14` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type CTCIF14_R = crate::BitReader;
#[doc = "Field `BRTIF14` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BRTIF14_R = crate::BitReader;
#[doc = "Field `BTIF14` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BTIF14_R = crate::BitReader;
#[doc = "Field `TCIF14` reader - channel x buffer transfer complete"]
pub type TCIF14_R = crate::BitReader;
#[doc = "Field `CRQA14` reader - channel x request active flag"]
pub type CRQA14_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif14(&self) -> TEIF14_R {
        TEIF14_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif14(&self) -> CTCIF14_R {
        CTCIF14_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif14(&self) -> BRTIF14_R {
        BRTIF14_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif14(&self) -> BTIF14_R {
        BTIF14_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif14(&self) -> TCIF14_R {
        TCIF14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa14(&self) -> CRQA14_R {
        CRQA14_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c14isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C14ISR_SPEC;
impl crate::RegisterSpec for MDMA_C14ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c14isr::R`](R) reader structure"]
impl crate::Readable for MDMA_C14ISR_SPEC {}
#[doc = "`reset()` method sets MDMA_C14ISR to value 0"]
impl crate::Resettable for MDMA_C14ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
