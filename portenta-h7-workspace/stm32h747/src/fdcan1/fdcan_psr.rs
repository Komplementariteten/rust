#[doc = "Register `FDCAN_PSR` reader"]
pub type R = crate::R<FDCAN_PSR_SPEC>;
#[doc = "Register `FDCAN_PSR` writer"]
pub type W = crate::W<FDCAN_PSR_SPEC>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ACT` reader - Activity"]
pub type ACT_R = crate::FieldReader;
#[doc = "Field `ACT` writer - Activity"]
pub type ACT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader;
#[doc = "Field `EP` writer - Error Passive"]
pub type EP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader;
#[doc = "Field `EW` writer - Warning Status"]
pub type EW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLEC` reader - Data Last Error Code"]
pub type DLEC_R = crate::FieldReader;
#[doc = "Field `DLEC` writer - Data Last Error Code"]
pub type DLEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN Message"]
pub type RESI_R = crate::BitReader;
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN Message"]
pub type RESI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN Message"]
pub type RBRS_R = crate::BitReader;
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN Message"]
pub type RBRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REDL` reader - Received FDCAN Message"]
pub type REDL_R = crate::BitReader;
#[doc = "Field `REDL` writer - Received FDCAN Message"]
pub type REDL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PXE` reader - Protocol Exception Event"]
pub type PXE_R = crate::BitReader;
#[doc = "Field `PXE` writer - Protocol Exception Event"]
pub type PXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TDCV_R = crate::FieldReader;
#[doc = "Field `TDCV` writer - Transmitter Delay Compensation Value"]
pub type TDCV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<FDCAN_PSR_SPEC, 0> {
        LEC_W::new(self)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    #[must_use]
    pub fn act(&mut self) -> ACT_W<FDCAN_PSR_SPEC, 3> {
        ACT_W::new(self)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<FDCAN_PSR_SPEC, 5> {
        EP_W::new(self)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<FDCAN_PSR_SPEC, 6> {
        EW_W::new(self)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<FDCAN_PSR_SPEC, 7> {
        BO_W::new(self)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DLEC_W<FDCAN_PSR_SPEC, 8> {
        DLEC_W::new(self)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<FDCAN_PSR_SPEC, 11> {
        RESI_W::new(self)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RBRS_W<FDCAN_PSR_SPEC, 12> {
        RBRS_W::new(self)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<FDCAN_PSR_SPEC, 13> {
        REDL_W::new(self)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PXE_W<FDCAN_PSR_SPEC, 14> {
        PXE_W::new(self)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    #[must_use]
    pub fn tdcv(&mut self) -> TDCV_W<FDCAN_PSR_SPEC, 16> {
        TDCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Protocol Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_PSR_SPEC;
impl crate::RegisterSpec for FDCAN_PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_psr::R`](R) reader structure"]
impl crate::Readable for FDCAN_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_psr::W`](W) writer structure"]
impl crate::Writable for FDCAN_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_PSR to value 0"]
impl crate::Resettable for FDCAN_PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
