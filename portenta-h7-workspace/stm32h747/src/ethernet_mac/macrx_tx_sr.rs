#[doc = "Register `MACRxTxSR` reader"]
pub type R = crate::R<MACRX_TX_SR_SPEC>;
#[doc = "Field `TJT` reader - TJT"]
pub type TJT_R = crate::BitReader;
#[doc = "Field `NCARR` reader - NCARR"]
pub type NCARR_R = crate::BitReader;
#[doc = "Field `LCARR` reader - LCARR"]
pub type LCARR_R = crate::BitReader;
#[doc = "Field `EXDEF` reader - EXDEF"]
pub type EXDEF_R = crate::BitReader;
#[doc = "Field `LCOL` reader - LCOL"]
pub type LCOL_R = crate::BitReader;
#[doc = "Field `EXCOL` reader - LCOL"]
pub type EXCOL_R = crate::BitReader;
#[doc = "Field `RWT` reader - RWT"]
pub type RWT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TJT"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NCARR"]
    #[inline(always)]
    pub fn ncarr(&self) -> NCARR_R {
        NCARR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCARR"]
    #[inline(always)]
    pub fn lcarr(&self) -> LCARR_R {
        LCARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXDEF"]
    #[inline(always)]
    pub fn exdef(&self) -> EXDEF_R {
        EXDEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCOL"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCOL"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - RWT"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Rx Tx status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrx_tx_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACRX_TX_SR_SPEC;
impl crate::RegisterSpec for MACRX_TX_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrx_tx_sr::R`](R) reader structure"]
impl crate::Readable for MACRX_TX_SR_SPEC {}
#[doc = "`reset()` method sets MACRxTxSR to value 0"]
impl crate::Resettable for MACRX_TX_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
