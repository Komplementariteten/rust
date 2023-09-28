#[doc = "Register `ICFR` writer"]
pub type W = crate::W<ICFR_SPEC>;
#[doc = "Field `CC1IF` writer - Clear COMP channel 1 Interrupt Flag"]
pub type CC1IF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CC2IF` writer - Clear COMP channel 2 Interrupt Flag"]
pub type CC2IF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 16 - Clear COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<ICFR_SPEC, 16> {
        CC1IF_W::new(self)
    }
    #[doc = "Bit 17 - Clear COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<ICFR_SPEC, 17> {
        CC2IF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Comparator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICFR_SPEC;
impl crate::RegisterSpec for ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icfr::W`](W) writer structure"]
impl crate::Writable for ICFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICFR to value 0"]
impl crate::Resettable for ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
