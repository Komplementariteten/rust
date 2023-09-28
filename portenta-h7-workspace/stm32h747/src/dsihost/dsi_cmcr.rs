#[doc = "Register `DSI_CMCR` reader"]
pub type R = crate::R<DSI_CMCR_SPEC>;
#[doc = "Register `DSI_CMCR` writer"]
pub type W = crate::W<DSI_CMCR_SPEC>;
#[doc = "Field `TEARE` reader - TEARE"]
pub type TEARE_R = crate::BitReader;
#[doc = "Field `TEARE` writer - TEARE"]
pub type TEARE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ARE` reader - ARE"]
pub type ARE_R = crate::BitReader;
#[doc = "Field `ARE` writer - ARE"]
pub type ARE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSW0TX` reader - GSW0TX"]
pub type GSW0TX_R = crate::BitReader;
#[doc = "Field `GSW0TX` writer - GSW0TX"]
pub type GSW0TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSW1TX` reader - GSW1TX"]
pub type GSW1TX_R = crate::BitReader;
#[doc = "Field `GSW1TX` writer - GSW1TX"]
pub type GSW1TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSW2TX` reader - GSW2TX"]
pub type GSW2TX_R = crate::BitReader;
#[doc = "Field `GSW2TX` writer - GSW2TX"]
pub type GSW2TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSR0TX` reader - GSR0TX"]
pub type GSR0TX_R = crate::BitReader;
#[doc = "Field `GSR0TX` writer - GSR0TX"]
pub type GSR0TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSR1TX` reader - GSR1TX"]
pub type GSR1TX_R = crate::BitReader;
#[doc = "Field `GSR1TX` writer - GSR1TX"]
pub type GSR1TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GSR2TX` reader - GSR2TX"]
pub type GSR2TX_R = crate::BitReader;
#[doc = "Field `GSR2TX` writer - GSR2TX"]
pub type GSR2TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GLWTX` reader - GLWTX"]
pub type GLWTX_R = crate::BitReader;
#[doc = "Field `GLWTX` writer - GLWTX"]
pub type GLWTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSW0TX` reader - DSW0TX"]
pub type DSW0TX_R = crate::BitReader;
#[doc = "Field `DSW0TX` writer - DSW0TX"]
pub type DSW0TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSW1TX` reader - DSW1TX"]
pub type DSW1TX_R = crate::BitReader;
#[doc = "Field `DSW1TX` writer - DSW1TX"]
pub type DSW1TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DSR0TX` reader - DSR0TX"]
pub type DSR0TX_R = crate::BitReader;
#[doc = "Field `DSR0TX` writer - DSR0TX"]
pub type DSR0TX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DLWTX` reader - DLWTX"]
pub type DLWTX_R = crate::BitReader;
#[doc = "Field `DLWTX` writer - DLWTX"]
pub type DLWTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MRDPS` reader - MRDPS"]
pub type MRDPS_R = crate::BitReader;
#[doc = "Field `MRDPS` writer - MRDPS"]
pub type MRDPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    #[must_use]
    pub fn teare(&mut self) -> TEARE_W<DSI_CMCR_SPEC, 0> {
        TEARE_W::new(self)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    #[must_use]
    pub fn are(&mut self) -> ARE_W<DSI_CMCR_SPEC, 1> {
        ARE_W::new(self)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<DSI_CMCR_SPEC, 8> {
        GSW0TX_W::new(self)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<DSI_CMCR_SPEC, 9> {
        GSW1TX_W::new(self)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<DSI_CMCR_SPEC, 10> {
        GSW2TX_W::new(self)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<DSI_CMCR_SPEC, 11> {
        GSR0TX_W::new(self)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<DSI_CMCR_SPEC, 12> {
        GSR1TX_W::new(self)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    #[must_use]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<DSI_CMCR_SPEC, 13> {
        GSR2TX_W::new(self)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    #[must_use]
    pub fn glwtx(&mut self) -> GLWTX_W<DSI_CMCR_SPEC, 14> {
        GLWTX_W::new(self)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    #[must_use]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<DSI_CMCR_SPEC, 16> {
        DSW0TX_W::new(self)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    #[must_use]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<DSI_CMCR_SPEC, 17> {
        DSW1TX_W::new(self)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    #[must_use]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<DSI_CMCR_SPEC, 18> {
        DSR0TX_W::new(self)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    #[must_use]
    pub fn dlwtx(&mut self) -> DLWTX_W<DSI_CMCR_SPEC, 19> {
        DLWTX_W::new(self)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    #[must_use]
    pub fn mrdps(&mut self) -> MRDPS_W<DSI_CMCR_SPEC, 24> {
        MRDPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host command mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_cmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_cmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CMCR_SPEC;
impl crate::RegisterSpec for DSI_CMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cmcr::R`](R) reader structure"]
impl crate::Readable for DSI_CMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_cmcr::W`](W) writer structure"]
impl crate::Writable for DSI_CMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_CMCR to value 0"]
impl crate::Resettable for DSI_CMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
