#[doc = "Register `DSI_PCR` reader"]
pub type R = crate::R<DSI_PCR_SPEC>;
#[doc = "Register `DSI_PCR` writer"]
pub type W = crate::W<DSI_PCR_SPEC>;
#[doc = "Field `ETTXE` reader - ETTXE"]
pub type ETTXE_R = crate::BitReader;
#[doc = "Field `ETTXE` writer - ETTXE"]
pub type ETTXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETRXE` reader - ETRXE"]
pub type ETRXE_R = crate::BitReader;
#[doc = "Field `ETRXE` writer - ETRXE"]
pub type ETRXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BTAE` reader - BTAE"]
pub type BTAE_R = crate::BitReader;
#[doc = "Field `BTAE` writer - BTAE"]
pub type BTAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ECCRXE` reader - ECCRXE"]
pub type ECCRXE_R = crate::BitReader;
#[doc = "Field `ECCRXE` writer - ECCRXE"]
pub type ECCRXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCRXE` reader - CRCRXE"]
pub type CRCRXE_R = crate::BitReader;
#[doc = "Field `CRCRXE` writer - CRCRXE"]
pub type CRCRXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    #[must_use]
    pub fn ettxe(&mut self) -> ETTXE_W<DSI_PCR_SPEC, 0> {
        ETTXE_W::new(self)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    #[must_use]
    pub fn etrxe(&mut self) -> ETRXE_W<DSI_PCR_SPEC, 1> {
        ETRXE_W::new(self)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    #[must_use]
    pub fn btae(&mut self) -> BTAE_W<DSI_PCR_SPEC, 2> {
        BTAE_W::new(self)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    #[must_use]
    pub fn eccrxe(&mut self) -> ECCRXE_W<DSI_PCR_SPEC, 3> {
        ECCRXE_W::new(self)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    #[must_use]
    pub fn crcrxe(&mut self) -> CRCRXE_W<DSI_PCR_SPEC, 4> {
        CRCRXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host protocol configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PCR_SPEC;
impl crate::RegisterSpec for DSI_PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pcr::R`](R) reader structure"]
impl crate::Readable for DSI_PCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_pcr::W`](W) writer structure"]
impl crate::Writable for DSI_PCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_PCR to value 0"]
impl crate::Resettable for DSI_PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
