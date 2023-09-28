#[doc = "Register `DSI_CR` reader"]
pub type R = crate::R<DSI_CR_SPEC>;
#[doc = "Register `DSI_CR` writer"]
pub type W = crate::W<DSI_CR_SPEC>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DSI_CR_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CR_SPEC;
impl crate::RegisterSpec for DSI_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cr::R`](R) reader structure"]
impl crate::Readable for DSI_CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_cr::W`](W) writer structure"]
impl crate::Writable for DSI_CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_CR to value 0"]
impl crate::Resettable for DSI_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
