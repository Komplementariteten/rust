#[doc = "Register `MTLTxQOMR` reader"]
pub type R = crate::R<MTLTX_QOMR_SPEC>;
#[doc = "Register `MTLTxQOMR` writer"]
pub type W = crate::W<MTLTX_QOMR_SPEC>;
#[doc = "Field `FTQ` reader - Flush Transmit Queue"]
pub type FTQ_R = crate::BitReader;
#[doc = "Field `FTQ` writer - Flush Transmit Queue"]
pub type FTQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSF` reader - Transmit Store and Forward"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit Store and Forward"]
pub type TSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXQEN` reader - Transmit Queue Enable"]
pub type TXQEN_R = crate::FieldReader;
#[doc = "Field `TTC` reader - Transmit Threshold Control"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit Threshold Control"]
pub type TTC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TQS` reader - Transmit Queue Size"]
pub type TQS_R = crate::FieldReader<u16>;
#[doc = "Field `TQS` writer - Transmit Queue Size"]
pub type TQS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bit 0 - Flush Transmit Queue"]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Store and Forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Transmit Queue Enable"]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:24 - Transmit Queue Size"]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Transmit Queue"]
    #[inline(always)]
    #[must_use]
    pub fn ftq(&mut self) -> FTQ_W<MTLTX_QOMR_SPEC, 0> {
        FTQ_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Store and Forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<MTLTX_QOMR_SPEC, 1> {
        TSF_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit Threshold Control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<MTLTX_QOMR_SPEC, 4> {
        TTC_W::new(self)
    }
    #[doc = "Bits 16:24 - Transmit Queue Size"]
    #[inline(always)]
    #[must_use]
    pub fn tqs(&mut self) -> TQS_W<MTLTX_QOMR_SPEC, 16> {
        TQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tx queue operating mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtltx_qomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLTX_QOMR_SPEC;
impl crate::RegisterSpec for MTLTX_QOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qomr::R`](R) reader structure"]
impl crate::Readable for MTLTX_QOMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtltx_qomr::W`](W) writer structure"]
impl crate::Writable for MTLTX_QOMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTLTxQOMR to value 0x0007_0008"]
impl crate::Resettable for MTLTX_QOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0007_0008;
}
