#[doc = "Register `DMACSR` reader"]
pub type R = crate::R<DMACSR_SPEC>;
#[doc = "Register `DMACSR` writer"]
pub type W = crate::W<DMACSR_SPEC>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable"]
pub type TBU_R = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable"]
pub type TBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RBU` reader - Receive Buffer Unavailable"]
pub type RBU_R = crate::BitReader;
#[doc = "Field `RBU` writer - Receive Buffer Unavailable"]
pub type RBU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RWT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ET` reader - Early Transmit Interrupt"]
pub type ET_R = crate::BitReader;
#[doc = "Field `ET` writer - Early Transmit Interrupt"]
pub type ET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ER` reader - Early Receive Interrupt"]
pub type ER_R = crate::BitReader;
#[doc = "Field `ER` writer - Early Receive Interrupt"]
pub type ER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBE` reader - Fatal Bus Error"]
pub type FBE_R = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error"]
pub type FBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDE` reader - Context Descriptor Error"]
pub type CDE_R = crate::BitReader;
#[doc = "Field `CDE` writer - Context Descriptor Error"]
pub type CDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TEB` reader - Tx DMA Error Bits"]
pub type TEB_R = crate::FieldReader;
#[doc = "Field `REB` reader - Rx DMA Error Bits"]
pub type REB_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RBU_R {
        RBU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn et(&self) -> ET_R {
        ET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn er(&self) -> ER_R {
        ER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&self) -> CDE_R {
        CDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    pub fn teb(&self) -> TEB_R {
        TEB_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    pub fn reb(&self) -> REB_R {
        REB_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TI_W<DMACSR_SPEC, 0> {
        TI_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<DMACSR_SPEC, 1> {
        TPS_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<DMACSR_SPEC, 2> {
        TBU_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RI_W<DMACSR_SPEC, 6> {
        RI_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RBU_W<DMACSR_SPEC, 7> {
        RBU_W::new(self)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<DMACSR_SPEC, 8> {
        RPS_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<DMACSR_SPEC, 9> {
        RWT_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn et(&mut self) -> ET_W<DMACSR_SPEC, 10> {
        ET_W::new(self)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn er(&mut self) -> ER_W<DMACSR_SPEC, 11> {
        ER_W::new(self)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn fbe(&mut self) -> FBE_W<DMACSR_SPEC, 12> {
        FBE_W::new(self)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CDE_W<DMACSR_SPEC, 13> {
        CDE_W::new(self)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AIS_W<DMACSR_SPEC, 14> {
        AIS_W::new(self)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NIS_W<DMACSR_SPEC, 15> {
        NIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACSR_SPEC;
impl crate::RegisterSpec for DMACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsr::R`](R) reader structure"]
impl crate::Readable for DMACSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacsr::W`](W) writer structure"]
impl crate::Writable for DMACSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACSR to value 0"]
impl crate::Resettable for DMACSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}