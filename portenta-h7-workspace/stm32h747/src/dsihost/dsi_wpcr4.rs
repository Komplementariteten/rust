#[doc = "Register `DSI_WPCR4` reader"]
pub type R = crate::R<DSI_WPCR4_SPEC>;
#[doc = "Register `DSI_WPCR4` writer"]
pub type W = crate::W<DSI_WPCR4_SPEC>;
#[doc = "Field `TCLKPOST` reader - TCLKPOST"]
pub type TCLKPOST_R = crate::FieldReader;
#[doc = "Field `TCLKPOST` writer - TCLKPOST"]
pub type TCLKPOST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TCLKPOST"]
    #[inline(always)]
    pub fn tclkpost(&self) -> TCLKPOST_R {
        TCLKPOST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TCLKPOST"]
    #[inline(always)]
    #[must_use]
    pub fn tclkpost(&mut self) -> TCLKPOST_W<DSI_WPCR4_SPEC, 0> {
        TCLKPOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WPCR4_SPEC;
impl crate::RegisterSpec for DSI_WPCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr4::R`](R) reader structure"]
impl crate::Readable for DSI_WPCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr4::W`](W) writer structure"]
impl crate::Writable for DSI_WPCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WPCR4 to value 0"]
impl crate::Resettable for DSI_WPCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
