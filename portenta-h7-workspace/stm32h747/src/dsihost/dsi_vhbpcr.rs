#[doc = "Register `DSI_VHBPCR` reader"]
pub type R = crate::R<DSI_VHBPCR_SPEC>;
#[doc = "Register `DSI_VHBPCR` writer"]
pub type W = crate::W<DSI_VHBPCR_SPEC>;
#[doc = "Field `HBP` reader - HBP"]
pub type HBP_R = crate::FieldReader<u16>;
#[doc = "Field `HBP` writer - HBP"]
pub type HBP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<DSI_VHBPCR_SPEC, 0> {
        HBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video HBP configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vhbpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VHBPCR_SPEC;
impl crate::RegisterSpec for DSI_VHBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhbpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VHBPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vhbpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VHBPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VHBPCR to value 0"]
impl crate::Resettable for DSI_VHBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
