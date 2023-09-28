#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `CRXBFF` writer - Clear receive buffer full flag"]
pub type CRXBFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTXBEF` writer - Clear transmit buffer empty flag"]
pub type CTXBEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRXBERF` writer - Clear receive CRC error flag"]
pub type CRXBERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRXOVRF` writer - Clear receive overrun error flag"]
pub type CRXOVRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTXUNRF` writer - Clear transmit underrun error flag"]
pub type CTXUNRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CTCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSRF` writer - Clear slave resume flag"]
pub type CSRF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRDYF` writer - Clear transceiver ready flag"]
pub type CRDYF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear receive buffer full flag"]
    #[inline(always)]
    #[must_use]
    pub fn crxbff(&mut self) -> CRXBFF_W<ICR_SPEC, 0> {
        CRXBFF_W::new(self)
    }
    #[doc = "Bit 1 - Clear transmit buffer empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctxbef(&mut self) -> CTXBEF_W<ICR_SPEC, 1> {
        CTXBEF_W::new(self)
    }
    #[doc = "Bit 2 - Clear receive CRC error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crxberf(&mut self) -> CRXBERF_W<ICR_SPEC, 2> {
        CRXBERF_W::new(self)
    }
    #[doc = "Bit 3 - Clear receive overrun error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crxovrf(&mut self) -> CRXOVRF_W<ICR_SPEC, 3> {
        CRXOVRF_W::new(self)
    }
    #[doc = "Bit 4 - Clear transmit underrun error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctxunrf(&mut self) -> CTXUNRF_W<ICR_SPEC, 4> {
        CTXUNRF_W::new(self)
    }
    #[doc = "Bit 7 - Clear transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<ICR_SPEC, 7> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 8 - Clear slave resume flag"]
    #[inline(always)]
    #[must_use]
    pub fn csrf(&mut self) -> CSRF_W<ICR_SPEC, 8> {
        CSRF_W::new(self)
    }
    #[doc = "Bit 11 - Clear transceiver ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn crdyf(&mut self) -> CRDYF_W<ICR_SPEC, 11> {
        CRDYF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SWPMI Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
