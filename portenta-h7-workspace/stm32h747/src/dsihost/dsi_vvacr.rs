#[doc = "Register `DSI_VVACR` reader"]
pub type R = crate::R<DSI_VVACR_SPEC>;
#[doc = "Register `DSI_VVACR` writer"]
pub type W = crate::W<DSI_VVACR_SPEC>;
#[doc = "Field `VA` reader - VA"]
pub type VA_R = crate::FieldReader<u16>;
#[doc = "Field `VA` writer - VA"]
pub type VA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<DSI_VVACR_SPEC, 0> {
        VA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video VA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vvacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVACR_SPEC;
impl crate::RegisterSpec for DSI_VVACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvacr::R`](R) reader structure"]
impl crate::Readable for DSI_VVACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvacr::W`](W) writer structure"]
impl crate::Writable for DSI_VVACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VVACR to value 0"]
impl crate::Resettable for DSI_VVACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
