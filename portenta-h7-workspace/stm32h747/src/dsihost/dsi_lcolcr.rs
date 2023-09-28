#[doc = "Register `DSI_LCOLCR` reader"]
pub type R = crate::R<DSI_LCOLCR_SPEC>;
#[doc = "Register `DSI_LCOLCR` writer"]
pub type W = crate::W<DSI_LCOLCR_SPEC>;
#[doc = "Field `COLC` reader - COLC"]
pub type COLC_R = crate::FieldReader;
#[doc = "Field `COLC` writer - COLC"]
pub type COLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `LPE` reader - LPE"]
pub type LPE_R = crate::BitReader;
#[doc = "Field `LPE` writer - LPE"]
pub type LPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    #[must_use]
    pub fn colc(&mut self) -> COLC_W<DSI_LCOLCR_SPEC, 0> {
        COLC_W::new(self)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    #[must_use]
    pub fn lpe(&mut self) -> LPE_W<DSI_LCOLCR_SPEC, 8> {
        LPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host LTDC color coding register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_lcolcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_lcolcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_LCOLCR_SPEC;
impl crate::RegisterSpec for DSI_LCOLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcolcr::R`](R) reader structure"]
impl crate::Readable for DSI_LCOLCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_lcolcr::W`](W) writer structure"]
impl crate::Writable for DSI_LCOLCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_LCOLCR to value 0"]
impl crate::Resettable for DSI_LCOLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
