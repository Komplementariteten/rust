#[doc = "Register `MDMA_C6ISR` reader"]
pub type R = crate::R<MDMA_C6ISR_SPEC>;
#[doc = "Field `TEIF6` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type TEIF6_R = crate::BitReader;
#[doc = "Field `CTCIF6` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type CTCIF6_R = crate::BitReader;
#[doc = "Field `BRTIF6` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BRTIF6_R = crate::BitReader;
#[doc = "Field `BTIF6` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BTIF6_R = crate::BitReader;
#[doc = "Field `TCIF6` reader - channel x buffer transfer complete"]
pub type TCIF6_R = crate::BitReader;
#[doc = "Field `CRQA6` reader - channel x request active flag"]
pub type CRQA6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif6(&self) -> BRTIF6_R {
        BRTIF6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif6(&self) -> BTIF6_R {
        BTIF6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa6(&self) -> CRQA6_R {
        CRQA6_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c6isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C6ISR_SPEC;
impl crate::RegisterSpec for MDMA_C6ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c6isr::R`](R) reader structure"]
impl crate::Readable for MDMA_C6ISR_SPEC {}
#[doc = "`reset()` method sets MDMA_C6ISR to value 0"]
impl crate::Resettable for MDMA_C6ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
