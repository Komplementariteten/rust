#[doc = "Register `DSI_WIER` reader"]
pub type R = crate::R<DSI_WIER_SPEC>;
#[doc = "Register `DSI_WIER` writer"]
pub type W = crate::W<DSI_WIER_SPEC>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERIE` reader - ERIE"]
pub type ERIE_R = crate::BitReader;
#[doc = "Field `ERIE` writer - ERIE"]
pub type ERIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLLIE` reader - PLLLIE"]
pub type PLLLIE_R = crate::BitReader;
#[doc = "Field `PLLLIE` writer - PLLLIE"]
pub type PLLLIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLUIE` reader - PLLUIE"]
pub type PLLUIE_R = crate::BitReader;
#[doc = "Field `PLLUIE` writer - PLLUIE"]
pub type PLLUIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RRIE` reader - RRIE"]
pub type RRIE_R = crate::BitReader;
#[doc = "Field `RRIE` writer - RRIE"]
pub type RRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERIE"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLLIE"]
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLLUIE"]
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - RRIE"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<DSI_WIER_SPEC, 0> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 1 - ERIE"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<DSI_WIER_SPEC, 1> {
        ERIE_W::new(self)
    }
    #[doc = "Bit 9 - PLLLIE"]
    #[inline(always)]
    #[must_use]
    pub fn plllie(&mut self) -> PLLLIE_W<DSI_WIER_SPEC, 9> {
        PLLLIE_W::new(self)
    }
    #[doc = "Bit 10 - PLLUIE"]
    #[inline(always)]
    #[must_use]
    pub fn plluie(&mut self) -> PLLUIE_W<DSI_WIER_SPEC, 10> {
        PLLUIE_W::new(self)
    }
    #[doc = "Bit 13 - RRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<DSI_WIER_SPEC, 13> {
        RRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WIER_SPEC;
impl crate::RegisterSpec for DSI_WIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wier::R`](R) reader structure"]
impl crate::Readable for DSI_WIER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wier::W`](W) writer structure"]
impl crate::Writable for DSI_WIER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WIER to value 0"]
impl crate::Resettable for DSI_WIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
