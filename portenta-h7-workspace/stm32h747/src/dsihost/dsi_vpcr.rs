#[doc = "Register `DSI_VPCR` reader"]
pub type R = crate::R<DSI_VPCR_SPEC>;
#[doc = "Register `DSI_VPCR` writer"]
pub type W = crate::W<DSI_VPCR_SPEC>;
#[doc = "Field `VPSIZE` reader - VPSIZE"]
pub type VPSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `VPSIZE` writer - VPSIZE"]
pub type VPSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - VPSIZE"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - VPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<DSI_VPCR_SPEC, 0> {
        VPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VPCR_SPEC;
impl crate::RegisterSpec for DSI_VPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VPCR to value 0"]
impl crate::Resettable for DSI_VPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
