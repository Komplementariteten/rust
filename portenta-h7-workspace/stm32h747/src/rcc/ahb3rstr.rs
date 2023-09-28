#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<AHB3RSTR_SPEC>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<AHB3RSTR_SPEC>;
#[doc = "Field `MDMARST` reader - MDMA block reset"]
pub type MDMARST_R = crate::BitReader;
#[doc = "Field `MDMARST` writer - MDMA block reset"]
pub type MDMARST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2DRST` reader - DMA2D block reset"]
pub type DMA2DRST_R = crate::BitReader;
#[doc = "Field `DMA2DRST` writer - DMA2D block reset"]
pub type DMA2DRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JPGDECRST` reader - JPGDEC block reset"]
pub type JPGDECRST_R = crate::BitReader;
#[doc = "Field `JPGDECRST` writer - JPGDEC block reset"]
pub type JPGDECRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FMCRST` reader - FMC block reset"]
pub type FMCRST_R = crate::BitReader;
#[doc = "Field `FMCRST` writer - FMC block reset"]
pub type FMCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QSPIRST` reader - QUADSPI and QUADSPI delay block reset"]
pub type QSPIRST_R = crate::BitReader;
#[doc = "Field `QSPIRST` writer - QUADSPI and QUADSPI delay block reset"]
pub type QSPIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay block reset"]
pub type SDMMC1RST_R = crate::BitReader;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay block reset"]
pub type SDMMC1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPURST` reader - CPU reset"]
pub type CPURST_R = crate::BitReader;
#[doc = "Field `CPURST` writer - CPU reset"]
pub type CPURST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> DMA2DRST_R {
        DMA2DRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC block reset"]
    #[inline(always)]
    pub fn jpgdecrst(&self) -> JPGDECRST_R {
        JPGDECRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI delay block reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    pub fn cpurst(&self) -> CPURST_R {
        CPURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    #[must_use]
    pub fn mdmarst(&mut self) -> MDMARST_W<AHB3RSTR_SPEC, 0> {
        MDMARST_W::new(self)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2drst(&mut self) -> DMA2DRST_W<AHB3RSTR_SPEC, 4> {
        DMA2DRST_W::new(self)
    }
    #[doc = "Bit 5 - JPGDEC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn jpgdecrst(&mut self) -> JPGDECRST_W<AHB3RSTR_SPEC, 5> {
        JPGDECRST_W::new(self)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fmcrst(&mut self) -> FMCRST_W<AHB3RSTR_SPEC, 12> {
        FMCRST_W::new(self)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn qspirst(&mut self) -> QSPIRST_W<AHB3RSTR_SPEC, 14> {
        QSPIRST_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<AHB3RSTR_SPEC, 16> {
        SDMMC1RST_W::new(self)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    #[must_use]
    pub fn cpurst(&mut self) -> CPURST_W<AHB3RSTR_SPEC, 31> {
        CPURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB3 Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB3RSTR_SPEC;
impl crate::RegisterSpec for AHB3RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for AHB3RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for AHB3RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for AHB3RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
