#[doc = "Register `DSI_LCCR` reader"]
pub type R = crate::R<DSI_LCCR_SPEC>;
#[doc = "Register `DSI_LCCR` writer"]
pub type W = crate::W<DSI_LCCR_SPEC>;
#[doc = "Field `CMDSIZE` reader - CMDSIZE"]
pub type CMDSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `CMDSIZE` writer - CMDSIZE"]
pub type CMDSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CMDSIZE"]
    #[inline(always)]
    pub fn cmdsize(&self) -> CMDSIZE_R {
        CMDSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CMDSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdsize(&mut self) -> CMDSIZE_W<DSI_LCCR_SPEC, 0> {
        CMDSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host LTDC command configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LCCR_SPEC;
impl crate::RegisterSpec for DSI_LCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lccr::R`](R) reader structure"]
impl crate::Readable for DSI_LCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_lccr::W`](W) writer structure"]
impl crate::Writable for DSI_LCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_LCCR to value 0"]
impl crate::Resettable for DSI_LCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
