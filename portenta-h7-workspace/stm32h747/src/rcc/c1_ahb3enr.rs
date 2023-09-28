#[doc = "Register `C1_AHB3ENR` reader"]
pub type R = crate::R<C1_AHB3ENR_SPEC>;
#[doc = "Register `C1_AHB3ENR` writer"]
pub type W = crate::W<C1_AHB3ENR_SPEC>;
#[doc = "Field `MDMAEN` reader - MDMA Peripheral Clock Enable"]
pub type MDMAEN_R = crate::BitReader;
#[doc = "Field `MDMAEN` writer - MDMA Peripheral Clock Enable"]
pub type MDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2DEN` reader - DMA2D Peripheral Clock Enable"]
pub type DMA2DEN_R = crate::BitReader;
#[doc = "Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable"]
pub type DMA2DEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JPGDECEN` reader - JPGDEC Peripheral Clock Enable"]
pub type JPGDECEN_R = crate::BitReader;
#[doc = "Field `JPGDECEN` writer - JPGDEC Peripheral Clock Enable"]
pub type JPGDECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMCEN` reader - FMC Peripheral Clocks Enable"]
pub type FMCEN_R = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMC Peripheral Clocks Enable"]
pub type FMCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPIEN` reader - QUADSPI and QUADSPI Delay Clock Enable"]
pub type QSPIEN_R = crate::BitReader;
#[doc = "Field `QSPIEN` writer - QUADSPI and QUADSPI Delay Clock Enable"]
pub type QSPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub type SDMMC1EN_R = crate::BitReader;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub type SDMMC1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC Peripheral Clock Enable"]
    #[inline(always)]
    pub fn jpgdecen(&self) -> JPGDECEN_R {
        JPGDECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<C1_AHB3ENR_SPEC, 0> {
        MDMAEN_W::new(self)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<C1_AHB3ENR_SPEC, 4> {
        DMA2DEN_W::new(self)
    }
    #[doc = "Bit 5 - JPGDEC Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jpgdecen(&mut self) -> JPGDECEN_W<C1_AHB3ENR_SPEC, 5> {
        JPGDECEN_W::new(self)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<C1_AHB3ENR_SPEC, 12> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<C1_AHB3ENR_SPEC, 14> {
        QSPIEN_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<C1_AHB3ENR_SPEC, 16> {
        SDMMC1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB3 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb3enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb3enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB3ENR_SPEC;
impl crate::RegisterSpec for C1_AHB3ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb3enr::R`](R) reader structure"]
impl crate::Readable for C1_AHB3ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb3enr::W`](W) writer structure"]
impl crate::Writable for C1_AHB3ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB3ENR to value 0"]
impl crate::Resettable for C1_AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
