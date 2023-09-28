#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RXDMA` reader - Reception DMA enable"]
pub type RXDMA_R = crate::BitReader;
#[doc = "Field `RXDMA` writer - Reception DMA enable"]
pub type RXDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXDMA` reader - Transmission DMA enable"]
pub type TXDMA_R = crate::BitReader;
#[doc = "Field `TXDMA` writer - Transmission DMA enable"]
pub type TXDMA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXMODE` reader - Reception buffering mode"]
pub type RXMODE_R = crate::BitReader;
#[doc = "Field `RXMODE` writer - Reception buffering mode"]
pub type RXMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXMODE` reader - Transmission buffering mode"]
pub type TXMODE_R = crate::BitReader;
#[doc = "Field `TXMODE` writer - Transmission buffering mode"]
pub type TXMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPBK` reader - Loopback mode enable"]
pub type LPBK_R = crate::BitReader;
#[doc = "Field `LPBK` writer - Loopback mode enable"]
pub type LPBK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPACT` reader - Single wire protocol master interface activate"]
pub type SWPACT_R = crate::BitReader;
#[doc = "Field `SWPACT` writer - Single wire protocol master interface activate"]
pub type SWPACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DEACT` reader - Single wire protocol master interface deactivate"]
pub type DEACT_R = crate::BitReader;
#[doc = "Field `DEACT` writer - Single wire protocol master interface deactivate"]
pub type DEACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWPTEN` reader - Single wire protocol master transceiver enable"]
pub type SWPTEN_R = crate::BitReader;
#[doc = "Field `SWPTEN` writer - Single wire protocol master transceiver enable"]
pub type SWPTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Single wire protocol master interface activate"]
    #[inline(always)]
    pub fn swpact(&self) -> SWPACT_R {
        SWPACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Single wire protocol master transceiver enable"]
    #[inline(always)]
    pub fn swpten(&self) -> SWPTEN_R {
        SWPTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma(&mut self) -> RXDMA_W<CR_SPEC, 0> {
        RXDMA_W::new(self)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdma(&mut self) -> TXDMA_W<CR_SPEC, 1> {
        TXDMA_W::new(self)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<CR_SPEC, 2> {
        RXMODE_W::new(self)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<CR_SPEC, 3> {
        TXMODE_W::new(self)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LPBK_W<CR_SPEC, 4> {
        LPBK_W::new(self)
    }
    #[doc = "Bit 5 - Single wire protocol master interface activate"]
    #[inline(always)]
    #[must_use]
    pub fn swpact(&mut self) -> SWPACT_W<CR_SPEC, 5> {
        SWPACT_W::new(self)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    #[must_use]
    pub fn deact(&mut self) -> DEACT_W<CR_SPEC, 10> {
        DEACT_W::new(self)
    }
    #[doc = "Bit 11 - Single wire protocol master transceiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpten(&mut self) -> SWPTEN_W<CR_SPEC, 11> {
        SWPTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SWPMI Configuration/Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
