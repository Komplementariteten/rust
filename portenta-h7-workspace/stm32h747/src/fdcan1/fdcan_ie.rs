#[doc = "Register `FDCAN_IE` reader"]
pub type R = crate::R<FDCAN_IE_SPEC>;
#[doc = "Field `RF0NE` reader - Rx FIFO 0 New Message Enable"]
pub type RF0NE_R = crate::BitReader;
#[doc = "Field `RF0WE` reader - Rx FIFO 0 Full Enable"]
pub type RF0WE_R = crate::BitReader;
#[doc = "Field `RF0FE` reader - Rx FIFO 0 Full Enable"]
pub type RF0FE_R = crate::BitReader;
#[doc = "Field `RF0LE` reader - Rx FIFO 0 Message Lost Enable"]
pub type RF0LE_R = crate::BitReader;
#[doc = "Field `RF1NE` reader - Rx FIFO 1 New Message Enable"]
pub type RF1NE_R = crate::BitReader;
#[doc = "Field `RF1WE` reader - Rx FIFO 1 Watermark Reached Enable"]
pub type RF1WE_R = crate::BitReader;
#[doc = "Field `RF1FE` reader - Rx FIFO 1 Watermark Reached Enable"]
pub type RF1FE_R = crate::BitReader;
#[doc = "Field `RF1LE` reader - Rx FIFO 1 Message Lost Enable"]
pub type RF1LE_R = crate::BitReader;
#[doc = "Field `HPME` reader - High Priority Message Enable"]
pub type HPME_R = crate::BitReader;
#[doc = "Field `TCE` reader - Transmission Completed Enable"]
pub type TCE_R = crate::BitReader;
#[doc = "Field `TCFE` reader - Transmission Cancellation Finished Enable"]
pub type TCFE_R = crate::BitReader;
#[doc = "Field `TEFE` reader - Tx FIFO Empty Enable"]
pub type TEFE_R = crate::BitReader;
#[doc = "Field `TEFNE` reader - Tx Event FIFO New Entry Enable"]
pub type TEFNE_R = crate::BitReader;
#[doc = "Field `TEFWE` reader - Tx Event FIFO Watermark Reached Enable"]
pub type TEFWE_R = crate::BitReader;
#[doc = "Field `TEFFE` reader - Tx Event FIFO Full Enable"]
pub type TEFFE_R = crate::BitReader;
#[doc = "Field `TEFLE` reader - Tx Event FIFO Element Lost Enable"]
pub type TEFLE_R = crate::BitReader;
#[doc = "Field `TSWE` reader - Timestamp Wraparound Enable"]
pub type TSWE_R = crate::BitReader;
#[doc = "Field `MRAFE` reader - Message RAM Access Failure Enable"]
pub type MRAFE_R = crate::BitReader;
#[doc = "Field `TOOE` reader - Timeout Occurred Enable"]
pub type TOOE_R = crate::BitReader;
#[doc = "Field `DRXE` reader - Message stored to Dedicated Rx Buffer Enable"]
pub type DRXE_R = crate::BitReader;
#[doc = "Field `BECE` reader - Bit Error Corrected Interrupt Enable"]
pub type BECE_R = crate::BitReader;
#[doc = "Field `BEUE` reader - Bit Error Uncorrected Interrupt Enable"]
pub type BEUE_R = crate::BitReader;
#[doc = "Field `ELOE` reader - Error Logging Overflow Enable"]
pub type ELOE_R = crate::BitReader;
#[doc = "Field `EPE` reader - Error Passive Enable"]
pub type EPE_R = crate::BitReader;
#[doc = "Field `EWE` reader - Warning Status Enable"]
pub type EWE_R = crate::BitReader;
#[doc = "Field `BOE` reader - Bus_Off Status Enable"]
pub type BOE_R = crate::BitReader;
#[doc = "Field `WDIE` reader - Watchdog Interrupt Enable"]
pub type WDIE_R = crate::BitReader;
#[doc = "Field `PEAE` reader - Protocol Error in Arbitration Phase Enable"]
pub type PEAE_R = crate::BitReader;
#[doc = "Field `PEDE` reader - Protocol Error in Data Phase Enable"]
pub type PEDE_R = crate::BitReader;
#[doc = "Field `ARAE` reader - Access to Reserved Address Enable"]
pub type ARAE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn tefe(&self) -> TEFE_R {
        TEFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "FDCAN Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ie::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_IE_SPEC;
impl crate::RegisterSpec for FDCAN_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ie::R`](R) reader structure"]
impl crate::Readable for FDCAN_IE_SPEC {}
#[doc = "`reset()` method sets FDCAN_IE to value 0"]
impl crate::Resettable for FDCAN_IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
