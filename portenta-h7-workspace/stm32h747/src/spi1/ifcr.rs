#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCR_SPEC>;
#[doc = "Field `EOTC` writer - End Of Transfer flag clear"]
pub type EOTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXTFC` writer - Transmission Transfer Filled flag clear"]
pub type TXTFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UDRC` writer - Underrun flag clear"]
pub type UDRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVRC` writer - Overrun flag clear"]
pub type OVRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEC` writer - CRC Error flag clear"]
pub type CRCEC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear"]
pub type TIFREC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MODFC` writer - Mode Fault flag clear"]
pub type MODFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSERFC` writer - TSERFC flag clear"]
pub type TSERFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SUSPC` writer - SUSPend flag clear"]
pub type SUSPC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 3 - End Of Transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<IFCR_SPEC, 3> {
        EOTC_W::new(self)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<IFCR_SPEC, 4> {
        TXTFC_W::new(self)
    }
    #[doc = "Bit 5 - Underrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<IFCR_SPEC, 5> {
        UDRC_W::new(self)
    }
    #[doc = "Bit 6 - Overrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<IFCR_SPEC, 6> {
        OVRC_W::new(self)
    }
    #[doc = "Bit 7 - CRC Error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<IFCR_SPEC, 7> {
        CRCEC_W::new(self)
    }
    #[doc = "Bit 8 - TI frame format error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<IFCR_SPEC, 8> {
        TIFREC_W::new(self)
    }
    #[doc = "Bit 9 - Mode Fault flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<IFCR_SPEC, 9> {
        MODFC_W::new(self)
    }
    #[doc = "Bit 10 - TSERFC flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tserfc(&mut self) -> TSERFC_W<IFCR_SPEC, 10> {
        TSERFC_W::new(self)
    }
    #[doc = "Bit 11 - SUSPend flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<IFCR_SPEC, 11> {
        SUSPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt/Status Flags Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
