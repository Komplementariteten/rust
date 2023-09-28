#[doc = "Register `DSI_VMCR` reader"]
pub type R = crate::R<DSI_VMCR_SPEC>;
#[doc = "Register `DSI_VMCR` writer"]
pub type W = crate::W<DSI_VMCR_SPEC>;
#[doc = "Field `VMT` reader - VMT"]
pub type VMT_R = crate::FieldReader;
#[doc = "Field `VMT` writer - VMT"]
pub type VMT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPVSAE` reader - LPVSAE"]
pub type LPVSAE_R = crate::BitReader;
#[doc = "Field `LPVSAE` writer - LPVSAE"]
pub type LPVSAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPVBPE` reader - LPVBPE"]
pub type LPVBPE_R = crate::BitReader;
#[doc = "Field `LPVBPE` writer - LPVBPE"]
pub type LPVBPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPVFPE` reader - LPVFPE"]
pub type LPVFPE_R = crate::BitReader;
#[doc = "Field `LPVFPE` writer - LPVFPE"]
pub type LPVFPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPVAE` reader - LPVAE"]
pub type LPVAE_R = crate::BitReader;
#[doc = "Field `LPVAE` writer - LPVAE"]
pub type LPVAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPHBPE` reader - LPHBPE"]
pub type LPHBPE_R = crate::BitReader;
#[doc = "Field `LPHBPE` writer - LPHBPE"]
pub type LPHBPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPHFPE` reader - LPHFPE"]
pub type LPHFPE_R = crate::BitReader;
#[doc = "Field `LPHFPE` writer - LPHFPE"]
pub type LPHFPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FBTAAE` reader - FBTAAE"]
pub type FBTAAE_R = crate::BitReader;
#[doc = "Field `FBTAAE` writer - FBTAAE"]
pub type FBTAAE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPCE` reader - LPCE"]
pub type LPCE_R = crate::BitReader;
#[doc = "Field `LPCE` writer - LPCE"]
pub type LPCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGE` reader - PGE"]
pub type PGE_R = crate::BitReader;
#[doc = "Field `PGE` writer - PGE"]
pub type PGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGM` reader - PGM"]
pub type PGM_R = crate::BitReader;
#[doc = "Field `PGM` writer - PGM"]
pub type PGM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGO` reader - PGO"]
pub type PGO_R = crate::BitReader;
#[doc = "Field `PGO` writer - PGO"]
pub type PGO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&self) -> VMT_R {
        VMT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LPVSAE_R {
        LPVSAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LPVBPE_R {
        LPVBPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LPVFPE_R {
        LPVFPE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&self) -> LPVAE_R {
        LPVAE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LPHBPE_R {
        LPHBPE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPHFPE"]
    #[inline(always)]
    pub fn lphfpe(&self) -> LPHFPE_R {
        LPHFPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FBTAAE_R {
        FBTAAE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPCE"]
    #[inline(always)]
    pub fn lpce(&self) -> LPCE_R {
        LPCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PGE"]
    #[inline(always)]
    pub fn pge(&self) -> PGE_R {
        PGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - PGM"]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - PGO"]
    #[inline(always)]
    pub fn pgo(&self) -> PGO_R {
        PGO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    #[must_use]
    pub fn vmt(&mut self) -> VMT_W<DSI_VMCR_SPEC, 0> {
        VMT_W::new(self)
    }
    #[doc = "Bit 8 - LPVSAE"]
    #[inline(always)]
    #[must_use]
    pub fn lpvsae(&mut self) -> LPVSAE_W<DSI_VMCR_SPEC, 8> {
        LPVSAE_W::new(self)
    }
    #[doc = "Bit 9 - LPVBPE"]
    #[inline(always)]
    #[must_use]
    pub fn lpvbpe(&mut self) -> LPVBPE_W<DSI_VMCR_SPEC, 9> {
        LPVBPE_W::new(self)
    }
    #[doc = "Bit 10 - LPVFPE"]
    #[inline(always)]
    #[must_use]
    pub fn lpvfpe(&mut self) -> LPVFPE_W<DSI_VMCR_SPEC, 10> {
        LPVFPE_W::new(self)
    }
    #[doc = "Bit 11 - LPVAE"]
    #[inline(always)]
    #[must_use]
    pub fn lpvae(&mut self) -> LPVAE_W<DSI_VMCR_SPEC, 11> {
        LPVAE_W::new(self)
    }
    #[doc = "Bit 12 - LPHBPE"]
    #[inline(always)]
    #[must_use]
    pub fn lphbpe(&mut self) -> LPHBPE_W<DSI_VMCR_SPEC, 12> {
        LPHBPE_W::new(self)
    }
    #[doc = "Bit 13 - LPHFPE"]
    #[inline(always)]
    #[must_use]
    pub fn lphfpe(&mut self) -> LPHFPE_W<DSI_VMCR_SPEC, 13> {
        LPHFPE_W::new(self)
    }
    #[doc = "Bit 14 - FBTAAE"]
    #[inline(always)]
    #[must_use]
    pub fn fbtaae(&mut self) -> FBTAAE_W<DSI_VMCR_SPEC, 14> {
        FBTAAE_W::new(self)
    }
    #[doc = "Bit 15 - LPCE"]
    #[inline(always)]
    #[must_use]
    pub fn lpce(&mut self) -> LPCE_W<DSI_VMCR_SPEC, 15> {
        LPCE_W::new(self)
    }
    #[doc = "Bit 16 - PGE"]
    #[inline(always)]
    #[must_use]
    pub fn pge(&mut self) -> PGE_W<DSI_VMCR_SPEC, 16> {
        PGE_W::new(self)
    }
    #[doc = "Bit 20 - PGM"]
    #[inline(always)]
    #[must_use]
    pub fn pgm(&mut self) -> PGM_W<DSI_VMCR_SPEC, 20> {
        PGM_W::new(self)
    }
    #[doc = "Bit 24 - PGO"]
    #[inline(always)]
    #[must_use]
    pub fn pgo(&mut self) -> PGO_W<DSI_VMCR_SPEC, 24> {
        PGO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host video mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_vmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VMCR_SPEC;
impl crate::RegisterSpec for DSI_VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vmcr::R`](R) reader structure"]
impl crate::Readable for DSI_VMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_vmcr::W`](W) writer structure"]
impl crate::Writable for DSI_VMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_VMCR to value 0"]
impl crate::Resettable for DSI_VMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
