#[doc = "Register `MDMA_C12ISR` reader"]
pub type R = crate::R<MDMA_C12ISR_SPEC>;
#[doc = "Field `TEIF12` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type TEIF12_R = crate::BitReader;
#[doc = "Field `CTCIF12` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type CTCIF12_R = crate::BitReader;
#[doc = "Field `BRTIF12` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BRTIF12_R = crate::BitReader;
#[doc = "Field `BTIF12` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BTIF12_R = crate::BitReader;
#[doc = "Field `TCIF12` reader - channel x buffer transfer complete"]
pub type TCIF12_R = crate::BitReader;
#[doc = "Field `CRQA12` reader - channel x request active flag"]
pub type CRQA12_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif12(&self) -> TEIF12_R {
        TEIF12_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif12(&self) -> CTCIF12_R {
        CTCIF12_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif12(&self) -> BRTIF12_R {
        BRTIF12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif12(&self) -> BTIF12_R {
        BTIF12_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif12(&self) -> TCIF12_R {
        TCIF12_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa12(&self) -> CRQA12_R {
        CRQA12_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c12isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C12ISR_SPEC;
impl crate::RegisterSpec for MDMA_C12ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c12isr::R`](R) reader structure"]
impl crate::Readable for MDMA_C12ISR_SPEC {}
#[doc = "`reset()` method sets MDMA_C12ISR to value 0"]
impl crate::Resettable for MDMA_C12ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
