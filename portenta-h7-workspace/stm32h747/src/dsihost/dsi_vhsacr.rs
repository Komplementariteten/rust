#[doc = "Register `DSI_VHSACR` reader"]
pub type R = crate::R<DSI_VHSACR_SPEC>;
#[doc = "Register `DSI_VHSACR` writer"]
pub type W = crate::W<DSI_VHSACR_SPEC>;
#[doc = "Field `HSA` reader - HSA"]
pub type HSA_R = crate::FieldReader<u16>;
#[doc = "Field `HSA` writer - HSA"]
pub type HSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    #[must_use]
    pub fn hsa(&mut self) -> HSA_W<DSI_VHSACR_SPEC, 0> {
        HSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video HSA configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vhsacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VHSACR_SPEC;
impl crate::RegisterSpec for DSI_VHSACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhsacr::R`](R) reader structure"]
impl crate::Readable for DSI_VHSACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vhsacr::W`](W) writer structure"]
impl crate::Writable for DSI_VHSACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VHSACR to value 0"]
impl crate::Resettable for DSI_VHSACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
