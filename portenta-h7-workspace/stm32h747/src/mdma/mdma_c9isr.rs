#[doc = "Register `MDMA_C9ISR` reader"]
pub type R = crate::R<MDMA_C9ISR_SPEC>;
#[doc = "Field `TEIF9` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type TEIF9_R = crate::BitReader;
#[doc = "Field `CTCIF9` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type CTCIF9_R = crate::BitReader;
#[doc = "Field `BRTIF9` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BRTIF9_R = crate::BitReader;
#[doc = "Field `BTIF9` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BTIF9_R = crate::BitReader;
#[doc = "Field `TCIF9` reader - channel x buffer transfer complete"]
pub type TCIF9_R = crate::BitReader;
#[doc = "Field `CRQA9` reader - channel x request active flag"]
pub type CRQA9_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif9(&self) -> TEIF9_R {
        TEIF9_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif9(&self) -> CTCIF9_R {
        CTCIF9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif9(&self) -> BRTIF9_R {
        BRTIF9_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif9(&self) -> BTIF9_R {
        BTIF9_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif9(&self) -> TCIF9_R {
        TCIF9_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa9(&self) -> CRQA9_R {
        CRQA9_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c9isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C9ISR_SPEC;
impl crate::RegisterSpec for MDMA_C9ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c9isr::R`](R) reader structure"]
impl crate::Readable for MDMA_C9ISR_SPEC {}
#[doc = "`reset()` method sets MDMA_C9ISR to value 0"]
impl crate::Resettable for MDMA_C9ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
