#[doc = "Register `DSI_VNPCR` reader"]
pub type R = crate::R<DSI_VNPCR_SPEC>;
#[doc = "Register `DSI_VNPCR` writer"]
pub type W = crate::W<DSI_VNPCR_SPEC>;
#[doc = "Field `NPSIZE` reader - NPSIZE"]
pub type NPSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `NPSIZE` writer - NPSIZE"]
pub type NPSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn npsize(&mut self) -> NPSIZE_W<DSI_VNPCR_SPEC, 0> {
        NPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video null packet configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vnpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VNPCR_SPEC;
impl crate::RegisterSpec for DSI_VNPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vnpcr::R`](R) reader structure"]
impl crate::Readable for DSI_VNPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vnpcr::W`](W) writer structure"]
impl crate::Writable for DSI_VNPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VNPCR to value 0"]
impl crate::Resettable for DSI_VNPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
