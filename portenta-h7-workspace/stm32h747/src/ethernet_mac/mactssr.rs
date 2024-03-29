#[doc = "Register `MACTSSR` reader"]
pub type R = crate::R<MACTSSR_SPEC>;
#[doc = "Field `TSSOVF` reader - TSSOVF"]
pub type TSSOVF_R = crate::BitReader;
#[doc = "Field `TSTARGT0` reader - TSTARGT0"]
pub type TSTARGT0_R = crate::BitReader;
#[doc = "Field `AUXTSTRIG` reader - AUXTSTRIG"]
pub type AUXTSTRIG_R = crate::BitReader;
#[doc = "Field `TSTRGTERR0` reader - TSTRGTERR0"]
pub type TSTRGTERR0_R = crate::BitReader;
#[doc = "Field `TXTSSIS` reader - TXTSSIS"]
pub type TXTSSIS_R = crate::BitReader;
#[doc = "Field `ATSSTN` reader - ATSSTN"]
pub type ATSSTN_R = crate::FieldReader;
#[doc = "Field `ATSSTM` reader - ATSSTM"]
pub type ATSSTM_R = crate::BitReader;
#[doc = "Field `ATSNS` reader - ATSNS"]
pub type ATSNS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TSSOVF"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSTARGT0"]
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUXTSTRIG"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSTRGTERR0"]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - TXTSSIS"]
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - ATSSTN"]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - ATSSTM"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - ATSNS"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
#[doc = "Timestamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSSR_SPEC;
impl crate::RegisterSpec for MACTSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactssr::R`](R) reader structure"]
impl crate::Readable for MACTSSR_SPEC {}
#[doc = "`reset()` method sets MACTSSR to value 0"]
impl crate::Resettable for MACTSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
