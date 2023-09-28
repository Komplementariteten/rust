#[doc = "Register `DSI_LCVCIDR` reader"]
pub type R = crate::R<DSI_LCVCIDR_SPEC>;
#[doc = "Register `DSI_LCVCIDR` writer"]
pub type W = crate::W<DSI_LCVCIDR_SPEC>;
#[doc = "Field `VCID` reader - VCID"]
pub type VCID_R = crate::FieldReader;
#[doc = "Field `VCID` writer - VCID"]
pub type VCID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<DSI_LCVCIDR_SPEC, 0> {
        VCID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host LTDC current VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcvcidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lcvcidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LCVCIDR_SPEC;
impl crate::RegisterSpec for DSI_LCVCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcvcidr::R`](R) reader structure"]
impl crate::Readable for DSI_LCVCIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_lcvcidr::W`](W) writer structure"]
impl crate::Writable for DSI_LCVCIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_LCVCIDR to value 0"]
impl crate::Resettable for DSI_LCVCIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
