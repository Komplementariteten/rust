#[doc = "Register `CONFR0` writer"]
pub type W = crate::W<CONFR0_SPEC>;
#[doc = "Field `START` writer - Start This bit start or stop the encoding or decoding process. Read this register always return 0."]
pub type START_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Start This bit start or stop the encoding or decoding process. Read this register always return 0."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CONFR0_SPEC, 0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "JPEG codec control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFR0_SPEC;
impl crate::RegisterSpec for CONFR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`confr0::W`](W) writer structure"]
impl crate::Writable for CONFR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFR0 to value 0"]
impl crate::Resettable for CONFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
