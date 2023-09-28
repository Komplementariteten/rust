#[doc = "Register `MACHWF1R` reader"]
pub type R = crate::R<MACHWF1R_SPEC>;
#[doc = "Field `RXFIFOSIZE` reader - RXFIFOSIZE"]
pub type RXFIFOSIZE_R = crate::FieldReader;
#[doc = "Field `TXFIFOSIZE` reader - TXFIFOSIZE"]
pub type TXFIFOSIZE_R = crate::FieldReader;
#[doc = "Field `OSTEN` reader - OSTEN"]
pub type OSTEN_R = crate::BitReader;
#[doc = "Field `PTOEN` reader - PTOEN"]
pub type PTOEN_R = crate::BitReader;
#[doc = "Field `ADVTHWORD` reader - ADVTHWORD"]
pub type ADVTHWORD_R = crate::BitReader;
#[doc = "Field `DCBEN` reader - DCBEN"]
pub type DCBEN_R = crate::BitReader;
#[doc = "Field `SPHEN` reader - SPHEN"]
pub type SPHEN_R = crate::BitReader;
#[doc = "Field `TSOEN` reader - TSOEN"]
pub type TSOEN_R = crate::BitReader;
#[doc = "Field `DBGMEMA` reader - DBGMEMA"]
pub type DBGMEMA_R = crate::BitReader;
#[doc = "Field `AVSEL` reader - AVSEL"]
pub type AVSEL_R = crate::BitReader;
#[doc = "Field `HASHTBLSZ` reader - HASHTBLSZ"]
pub type HASHTBLSZ_R = crate::FieldReader;
#[doc = "Field `L3L4FNUM` reader - L3L4FNUM"]
pub type L3L4FNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - RXFIFOSIZE"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - TXFIFOSIZE"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - OSTEN"]
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADVTHWORD"]
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DCBEN"]
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPHEN"]
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TSOEN"]
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBGMEMA"]
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AVSEL"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25 - HASHTBLSZ"]
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:30 - L3L4FNUM"]
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3L4FNUM_R {
        L3L4FNUM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[doc = "HW feature 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machwf1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHWF1R_SPEC;
impl crate::RegisterSpec for MACHWF1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machwf1r::R`](R) reader structure"]
impl crate::Readable for MACHWF1R_SPEC {}
#[doc = "`reset()` method sets MACHWF1R to value 0x1184_1904"]
impl crate::Resettable for MACHWF1R_SPEC {
    const RESET_VALUE: Self::Ux = 0x1184_1904;
}
