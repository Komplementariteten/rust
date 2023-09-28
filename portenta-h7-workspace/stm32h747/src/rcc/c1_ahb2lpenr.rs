#[doc = "Register `C1_AHB2LPENR` reader"]
pub type R = crate::R<C1_AHB2LPENR_SPEC>;
#[doc = "Register `C1_AHB2LPENR` writer"]
pub type W = crate::W<C1_AHB2LPENR_SPEC>;
#[doc = "Field `CAMITFLPEN` reader - CAMITF peripheral clock enable during CSleep mode"]
pub type CAMITFLPEN_R = crate::BitReader;
#[doc = "Field `CAMITFLPEN` writer - CAMITF peripheral clock enable during CSleep mode"]
pub type CAMITFLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode"]
pub type CRYPTLPEN_R = crate::BitReader;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode"]
pub type CRYPTLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode"]
pub type HASHLPEN_R = crate::BitReader;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode"]
pub type HASHLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode"]
pub type RNGLPEN_R = crate::BitReader;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode"]
pub type RNGLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub type SDMMC2LPEN_R = crate::BitReader;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub type SDMMC2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 Clock Enable During CSleep Mode"]
pub type SRAM1LPEN_R = crate::BitReader;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 Clock Enable During CSleep Mode"]
pub type SRAM1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 Clock Enable During CSleep Mode"]
pub type SRAM2LPEN_R = crate::BitReader;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 Clock Enable During CSleep Mode"]
pub type SRAM2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM3LPEN` reader - SRAM3 Clock Enable During CSleep Mode"]
pub type SRAM3LPEN_R = crate::BitReader;
#[doc = "Field `SRAM3LPEN` writer - SRAM3 Clock Enable During CSleep Mode"]
pub type SRAM3LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAMITF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn camitflpen(&self) -> CAMITFLPEN_R {
        CAMITFLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn camitflpen(&mut self) -> CAMITFLPEN_W<C1_AHB2LPENR_SPEC, 0> {
        CAMITFLPEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W<C1_AHB2LPENR_SPEC, 4> {
        CRYPTLPEN_W::new(self)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<C1_AHB2LPENR_SPEC, 5> {
        HASHLPEN_W::new(self)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<C1_AHB2LPENR_SPEC, 6> {
        RNGLPEN_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<C1_AHB2LPENR_SPEC, 9> {
        SDMMC2LPEN_W::new(self)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<C1_AHB2LPENR_SPEC, 29> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W<C1_AHB2LPENR_SPEC, 30> {
        SRAM2LPEN_W::new(self)
    }
    #[doc = "Bit 31 - SRAM3 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W<C1_AHB2LPENR_SPEC, 31> {
        SRAM3LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB2LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb2lpenr::R`](R) reader structure"]
impl crate::Readable for C1_AHB2LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb2lpenr::W`](W) writer structure"]
impl crate::Writable for C1_AHB2LPENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB2LPENR to value 0"]
impl crate::Resettable for C1_AHB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
