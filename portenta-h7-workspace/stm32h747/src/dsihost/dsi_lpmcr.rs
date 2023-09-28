#[doc = "Register `DSI_LPMCR` reader"]
pub type R = crate::R<DSI_LPMCR_SPEC>;
#[doc = "Register `DSI_LPMCR` writer"]
pub type W = crate::W<DSI_LPMCR_SPEC>;
#[doc = "Field `VLPSIZE` reader - VLPSIZE"]
pub type VLPSIZE_R = crate::FieldReader;
#[doc = "Field `VLPSIZE` writer - VLPSIZE"]
pub type VLPSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `LPSIZE` reader - LPSIZE"]
pub type LPSIZE_R = crate::FieldReader;
#[doc = "Field `LPSIZE` writer - LPSIZE"]
pub type LPSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<DSI_LPMCR_SPEC, 0> {
        VLPSIZE_W::new(self)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn lpsize(&mut self) -> LPSIZE_W<DSI_LPMCR_SPEC, 16> {
        LPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host low-power mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lpmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lpmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LPMCR_SPEC;
impl crate::RegisterSpec for DSI_LPMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpmcr::R`](R) reader structure"]
impl crate::Readable for DSI_LPMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_lpmcr::W`](W) writer structure"]
impl crate::Writable for DSI_LPMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_LPMCR to value 0"]
impl crate::Resettable for DSI_LPMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
