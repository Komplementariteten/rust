#[doc = "Register `DMACTxCR` reader"]
pub type R = crate::R<DMACTX_CR_SPEC>;
#[doc = "Register `DMACTxCR` writer"]
pub type W = crate::W<DMACTX_CR_SPEC>;
#[doc = "Field `ST` reader - Start or Stop Transmission Command"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - Start or Stop Transmission Command"]
pub type ST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSF` reader - Operate on Second Packet"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on Second Packet"]
pub type OSF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSE` reader - TCP Segmentation Enabled"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - TCP Segmentation Enabled"]
pub type TSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXPBL` reader - Transmit Programmable Burst Length"]
pub type TXPBL_R = crate::FieldReader;
#[doc = "Field `TXPBL` writer - Transmit Programmable Burst Length"]
pub type TXPBL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<DMACTX_CR_SPEC, 0> {
        ST_W::new(self)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMACTX_CR_SPEC, 4> {
        OSF_W::new(self)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<DMACTX_CR_SPEC, 12> {
        TSE_W::new(self)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn txpbl(&mut self) -> TXPBL_W<DMACTX_CR_SPEC, 16> {
        TXPBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactx_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactx_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACTX_CR_SPEC;
impl crate::RegisterSpec for DMACTX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactx_cr::R`](R) reader structure"]
impl crate::Readable for DMACTX_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmactx_cr::W`](W) writer structure"]
impl crate::Writable for DMACTX_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACTxCR to value 0"]
impl crate::Resettable for DMACTX_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
