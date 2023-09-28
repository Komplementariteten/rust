#[doc = "Register `FDCAN_TTOST` reader"]
pub type R = crate::R<FDCAN_TTOST_SPEC>;
#[doc = "Register `FDCAN_TTOST` writer"]
pub type W = crate::W<FDCAN_TTOST_SPEC>;
#[doc = "Field `EL` reader - Error Level"]
pub type EL_R = crate::FieldReader;
#[doc = "Field `EL` writer - Error Level"]
pub type EL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MS` reader - Master State."]
pub type MS_R = crate::FieldReader;
#[doc = "Field `MS` writer - Master State."]
pub type MS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SYS` reader - Synchronization State"]
pub type SYS_R = crate::FieldReader;
#[doc = "Field `SYS` writer - Synchronization State"]
pub type SYS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `GTP` reader - Quality of Global Time Phase"]
pub type GTP_R = crate::BitReader;
#[doc = "Field `GTP` writer - Quality of Global Time Phase"]
pub type GTP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QCS` reader - Quality of Clock Speed"]
pub type QCS_R = crate::BitReader;
#[doc = "Field `QCS` writer - Quality of Clock Speed"]
pub type QCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTO` reader - Reference Trigger Offset"]
pub type RTO_R = crate::FieldReader;
#[doc = "Field `RTO` writer - Reference Trigger Offset"]
pub type RTO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity"]
pub type WGTD_R = crate::BitReader;
#[doc = "Field `WGTD` writer - Wait for Global Time Discontinuity"]
pub type WGTD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GFI` reader - Gap Finished Indicator."]
pub type GFI_R = crate::BitReader;
#[doc = "Field `GFI` writer - Gap Finished Indicator."]
pub type GFI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TMP` reader - Time Master Priority"]
pub type TMP_R = crate::FieldReader;
#[doc = "Field `TMP` writer - Time Master Priority"]
pub type TMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `GSI` reader - Gap Started Indicator."]
pub type GSI_R = crate::BitReader;
#[doc = "Field `GSI` writer - Gap Started Indicator."]
pub type GSI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WFE` reader - Wait for Event"]
pub type WFE_R = crate::BitReader;
#[doc = "Field `WFE` writer - Wait for Event"]
pub type WFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWE` reader - Application Watchdog Event"]
pub type AWE_R = crate::BitReader;
#[doc = "Field `AWE` writer - Application Watchdog Event"]
pub type AWE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization"]
pub type WECS_R = crate::BitReader;
#[doc = "Field `WECS` writer - Wait for External Clock Synchronization"]
pub type WECS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPL` reader - Schedule Phase Lock"]
pub type SPL_R = crate::BitReader;
#[doc = "Field `SPL` writer - Schedule Phase Lock"]
pub type SPL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn gtp(&self) -> GTP_R {
        GTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    #[must_use]
    pub fn el(&mut self) -> EL_W<FDCAN_TTOST_SPEC, 0> {
        EL_W::new(self)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<FDCAN_TTOST_SPEC, 2> {
        MS_W::new(self)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    #[must_use]
    pub fn sys(&mut self) -> SYS_W<FDCAN_TTOST_SPEC, 4> {
        SYS_W::new(self)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    #[must_use]
    pub fn gtp(&mut self) -> GTP_W<FDCAN_TTOST_SPEC, 6> {
        GTP_W::new(self)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    #[must_use]
    pub fn qcs(&mut self) -> QCS_W<FDCAN_TTOST_SPEC, 7> {
        QCS_W::new(self)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    #[must_use]
    pub fn rto(&mut self) -> RTO_W<FDCAN_TTOST_SPEC, 8> {
        RTO_W::new(self)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    #[must_use]
    pub fn wgtd(&mut self) -> WGTD_W<FDCAN_TTOST_SPEC, 22> {
        WGTD_W::new(self)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    #[must_use]
    pub fn gfi(&mut self) -> GFI_W<FDCAN_TTOST_SPEC, 23> {
        GFI_W::new(self)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    #[must_use]
    pub fn tmp(&mut self) -> TMP_W<FDCAN_TTOST_SPEC, 24> {
        TMP_W::new(self)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    #[must_use]
    pub fn gsi(&mut self) -> GSI_W<FDCAN_TTOST_SPEC, 27> {
        GSI_W::new(self)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<FDCAN_TTOST_SPEC, 28> {
        WFE_W::new(self)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    #[must_use]
    pub fn awe(&mut self) -> AWE_W<FDCAN_TTOST_SPEC, 29> {
        AWE_W::new(self)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn wecs(&mut self) -> WECS_W<FDCAN_TTOST_SPEC, 30> {
        WECS_W::new(self)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SPL_W<FDCAN_TTOST_SPEC, 31> {
        SPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN TT Operation Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttost::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttost::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTOST_SPEC;
impl crate::RegisterSpec for FDCAN_TTOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttost::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTOST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttost::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTOST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTOST to value 0"]
impl crate::Resettable for FDCAN_TTOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
