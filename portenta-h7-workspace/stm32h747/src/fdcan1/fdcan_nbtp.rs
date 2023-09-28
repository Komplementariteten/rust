#[doc = "Register `FDCAN_NBTP` reader"]
pub type R = crate::R<FDCAN_NBTP_SPEC>;
#[doc = "Register `FDCAN_NBTP` writer"]
pub type W = crate::W<FDCAN_NBTP_SPEC>;
#[doc = "Field `TSEG2` reader - Nominal Time segment after sample point"]
pub type TSEG2_R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Nominal Time segment after sample point"]
pub type TSEG2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `NTSEG1` reader - Nominal Time segment before sample point"]
pub type NTSEG1_R = crate::FieldReader;
#[doc = "Field `NTSEG1` writer - Nominal Time segment before sample point"]
pub type NTSEG1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `NBRP` reader - Bit Rate Prescaler"]
pub type NBRP_R = crate::FieldReader<u16>;
#[doc = "Field `NBRP` writer - Bit Rate Prescaler"]
pub type NBRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `NSJW` reader - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NSJW_R = crate::FieldReader;
#[doc = "Field `NSJW` writer - NSJW: Nominal (Re)Synchronization Jump Width"]
pub type NSJW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time segment after sample point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<FDCAN_NBTP_SPEC, 0> {
        TSEG2_W::new(self)
    }
    #[doc = "Bits 8:15 - Nominal Time segment before sample point"]
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<FDCAN_NBTP_SPEC, 8> {
        NTSEG1_W::new(self)
    }
    #[doc = "Bits 16:24 - Bit Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<FDCAN_NBTP_SPEC, 16> {
        NBRP_W::new(self)
    }
    #[doc = "Bits 25:31 - NSJW: Nominal (Re)Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<FDCAN_NBTP_SPEC, 25> {
        NSJW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_nbtp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_nbtp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_NBTP_SPEC;
impl crate::RegisterSpec for FDCAN_NBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_nbtp::R`](R) reader structure"]
impl crate::Readable for FDCAN_NBTP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_nbtp::W`](W) writer structure"]
impl crate::Writable for FDCAN_NBTP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_NBTP to value 0"]
impl crate::Resettable for FDCAN_NBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
