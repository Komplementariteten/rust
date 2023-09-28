#[doc = "Register `C1_AHB2ENR` reader"]
pub type R = crate::R<C1_AHB2ENR_SPEC>;
#[doc = "Register `C1_AHB2ENR` writer"]
pub type W = crate::W<C1_AHB2ENR_SPEC>;
#[doc = "Field `CAMITFEN` reader - CAMITF peripheral clock enable"]
pub type CAMITFEN_R = crate::BitReader;
#[doc = "Field `CAMITFEN` writer - CAMITF peripheral clock enable"]
pub type CAMITFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRYPTEN` reader - CRYPT peripheral clock enable"]
pub type CRYPTEN_R = crate::BitReader;
#[doc = "Field `CRYPTEN` writer - CRYPT peripheral clock enable"]
pub type CRYPTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HASHEN` reader - HASH peripheral clock enable"]
pub type HASHEN_R = crate::BitReader;
#[doc = "Field `HASHEN` writer - HASH peripheral clock enable"]
pub type HASHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RNGEN` reader - RNG peripheral clocks enable"]
pub type RNGEN_R = crate::BitReader;
#[doc = "Field `RNGEN` writer - RNG peripheral clocks enable"]
pub type RNGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable"]
pub type SDMMC2EN_R = crate::BitReader;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable"]
pub type SDMMC2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM1EN` reader - SRAM1 block enable"]
pub type SRAM1EN_R = crate::BitReader;
#[doc = "Field `SRAM1EN` writer - SRAM1 block enable"]
pub type SRAM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM2EN` reader - SRAM2 block enable"]
pub type SRAM2EN_R = crate::BitReader;
#[doc = "Field `SRAM2EN` writer - SRAM2 block enable"]
pub type SRAM2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM3EN` reader - SRAM3 block enable"]
pub type SRAM3EN_R = crate::BitReader;
#[doc = "Field `SRAM3EN` writer - SRAM3 block enable"]
pub type SRAM3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAMITF peripheral clock enable"]
    #[inline(always)]
    pub fn camitfen(&self) -> CAMITFEN_R {
        CAMITFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&self) -> CRYPTEN_R {
        CRYPTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&self) -> SRAM2EN_R {
        SRAM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&self) -> SRAM3EN_R {
        SRAM3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn camitfen(&mut self) -> CAMITFEN_W<C1_AHB2ENR_SPEC, 0> {
        CAMITFEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crypten(&mut self) -> CRYPTEN_W<C1_AHB2ENR_SPEC, 4> {
        CRYPTEN_W::new(self)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<C1_AHB2ENR_SPEC, 5> {
        HASHEN_W::new(self)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<C1_AHB2ENR_SPEC, 6> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<C1_AHB2ENR_SPEC, 9> {
        SDMMC2EN_W::new(self)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram1en(&mut self) -> SRAM1EN_W<C1_AHB2ENR_SPEC, 29> {
        SRAM1EN_W::new(self)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram2en(&mut self) -> SRAM2EN_W<C1_AHB2ENR_SPEC, 30> {
        SRAM2EN_W::new(self)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram3en(&mut self) -> SRAM3EN_W<C1_AHB2ENR_SPEC, 31> {
        SRAM3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB2ENR_SPEC;
impl crate::RegisterSpec for C1_AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb2enr::R`](R) reader structure"]
impl crate::Readable for C1_AHB2ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb2enr::W`](W) writer structure"]
impl crate::Writable for C1_AHB2ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB2ENR to value 0"]
impl crate::Resettable for C1_AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
