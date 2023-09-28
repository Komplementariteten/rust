#[doc = "Register `MTLRxQOMR` reader"]
pub type R = crate::R<MTLRX_QOMR_SPEC>;
#[doc = "Register `MTLRxQOMR` writer"]
pub type W = crate::W<MTLRX_QOMR_SPEC>;
#[doc = "Field `RTC` reader - RTC"]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - RTC"]
pub type RTC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FUP` reader - FUP"]
pub type FUP_R = crate::BitReader;
#[doc = "Field `FUP` writer - FUP"]
pub type FUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEP` reader - FEP"]
pub type FEP_R = crate::BitReader;
#[doc = "Field `FEP` writer - FEP"]
pub type FEP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSF` reader - RSF"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - RSF"]
pub type RSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIS_TCP_EF` reader - DIS_TCP_EF"]
pub type DIS_TCP_EF_R = crate::BitReader;
#[doc = "Field `DIS_TCP_EF` writer - DIS_TCP_EF"]
pub type DIS_TCP_EF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EHFC` reader - EHFC"]
pub type EHFC_R = crate::BitReader;
#[doc = "Field `EHFC` writer - EHFC"]
pub type EHFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFA` reader - RFA"]
pub type RFA_R = crate::FieldReader;
#[doc = "Field `RFA` writer - RFA"]
pub type RFA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RFD` reader - RFD"]
pub type RFD_R = crate::FieldReader;
#[doc = "Field `RFD` writer - RFD"]
pub type RFD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RQS` reader - RQS"]
pub type RQS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    pub fn fep(&self) -> FEP_R {
        FEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    pub fn dis_tcp_ef(&self) -> DIS_TCP_EF_R {
        DIS_TCP_EF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    pub fn ehfc(&self) -> EHFC_R {
        EHFC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    pub fn rfa(&self) -> RFA_R {
        RFA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    pub fn rfd(&self) -> RFD_R {
        RFD_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 20:22 - RQS"]
    #[inline(always)]
    pub fn rqs(&self) -> RQS_R {
        RQS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<MTLRX_QOMR_SPEC, 0> {
        RTC_W::new(self)
    }
    #[doc = "Bit 3 - FUP"]
    #[inline(always)]
    #[must_use]
    pub fn fup(&mut self) -> FUP_W<MTLRX_QOMR_SPEC, 3> {
        FUP_W::new(self)
    }
    #[doc = "Bit 4 - FEP"]
    #[inline(always)]
    #[must_use]
    pub fn fep(&mut self) -> FEP_W<MTLRX_QOMR_SPEC, 4> {
        FEP_W::new(self)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<MTLRX_QOMR_SPEC, 5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 6 - DIS_TCP_EF"]
    #[inline(always)]
    #[must_use]
    pub fn dis_tcp_ef(&mut self) -> DIS_TCP_EF_W<MTLRX_QOMR_SPEC, 6> {
        DIS_TCP_EF_W::new(self)
    }
    #[doc = "Bit 7 - EHFC"]
    #[inline(always)]
    #[must_use]
    pub fn ehfc(&mut self) -> EHFC_W<MTLRX_QOMR_SPEC, 7> {
        EHFC_W::new(self)
    }
    #[doc = "Bits 8:10 - RFA"]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RFA_W<MTLRX_QOMR_SPEC, 8> {
        RFA_W::new(self)
    }
    #[doc = "Bits 14:16 - RFD"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RFD_W<MTLRX_QOMR_SPEC, 14> {
        RFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Rx queue operating mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlrx_qomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QOMR_SPEC;
impl crate::RegisterSpec for MTLRX_QOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qomr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QOMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtlrx_qomr::W`](W) writer structure"]
impl crate::Writable for MTLRX_QOMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLRxQOMR to value 0x0070_0000"]
impl crate::Resettable for MTLRX_QOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0070_0000;
}
