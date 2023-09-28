#[doc = "Register `DSI_CCR` reader"]
pub type R = crate::R<DSI_CCR_SPEC>;
#[doc = "Register `DSI_CCR` writer"]
pub type W = crate::W<DSI_CCR_SPEC>;
#[doc = "Field `TXECKDIV` reader - TXECKDIV"]
pub type TXECKDIV_R = crate::FieldReader;
#[doc = "Field `TXECKDIV` writer - TXECKDIV"]
pub type TXECKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TOCKDIV` reader - TOCKDIV"]
pub type TOCKDIV_R = crate::FieldReader;
#[doc = "Field `TOCKDIV` writer - TOCKDIV"]
pub type TOCKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&self) -> TXECKDIV_R {
        TXECKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&self) -> TOCKDIV_R {
        TOCKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn txeckdiv(&mut self) -> TXECKDIV_W<DSI_CCR_SPEC, 0> {
        TXECKDIV_W::new(self)
    }
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn tockdiv(&mut self) -> TOCKDIV_W<DSI_CCR_SPEC, 8> {
        TOCKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CCR_SPEC;
impl crate::RegisterSpec for DSI_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ccr::R`](R) reader structure"]
impl crate::Readable for DSI_CCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_ccr::W`](W) writer structure"]
impl crate::Writable for DSI_CCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_CCR to value 0"]
impl crate::Resettable for DSI_CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
