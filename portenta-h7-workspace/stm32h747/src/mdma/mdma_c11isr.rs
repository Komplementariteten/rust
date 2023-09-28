#[doc = "Register `MDMA_C11ISR` reader"]
pub type R = crate::R<MDMA_C11ISR_SPEC>;
#[doc = "Field `TEIF11` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type TEIF11_R = crate::BitReader;
#[doc = "Field `CTCIF11` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type CTCIF11_R = crate::BitReader;
#[doc = "Field `BRTIF11` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BRTIF11_R = crate::BitReader;
#[doc = "Field `BTIF11` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type BTIF11_R = crate::BitReader;
#[doc = "Field `TCIF11` reader - channel x buffer transfer complete"]
pub type TCIF11_R = crate::BitReader;
#[doc = "Field `CRQA11` reader - channel x request active flag"]
pub type CRQA11_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif11(&self) -> TEIF11_R {
        TEIF11_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif11(&self) -> CTCIF11_R {
        CTCIF11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif11(&self) -> BRTIF11_R {
        BRTIF11_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif11(&self) -> BTIF11_R {
        BTIF11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif11(&self) -> TCIF11_R {
        TCIF11_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa11(&self) -> CRQA11_R {
        CRQA11_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c11isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C11ISR_SPEC;
impl crate::RegisterSpec for MDMA_C11ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c11isr::R`](R) reader structure"]
impl crate::Readable for MDMA_C11ISR_SPEC {}
#[doc = "`reset()` method sets MDMA_C11ISR to value 0"]
impl crate::Resettable for MDMA_C11ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
