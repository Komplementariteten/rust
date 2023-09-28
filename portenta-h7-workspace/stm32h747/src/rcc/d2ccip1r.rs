#[doc = "Register `D2CCIP1R` reader"]
pub type R = crate::R<D2CCIP1R_SPEC>;
#[doc = "Register `D2CCIP1R` writer"]
pub type W = crate::W<D2CCIP1R_SPEC>;
#[doc = "Field `SAI1SRC` reader - SAI1 and DFSDM1 kernel Aclk clock source selection"]
pub type SAI1SRC_R = crate::FieldReader;
#[doc = "Field `SAI1SRC` writer - SAI1 and DFSDM1 kernel Aclk clock source selection"]
pub type SAI1SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SAI23SRC` reader - SAI2 and SAI3 kernel clock source selection"]
pub type SAI23SRC_R = crate::FieldReader;
#[doc = "Field `SAI23SRC` writer - SAI2 and SAI3 kernel clock source selection"]
pub type SAI23SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI123SRC` reader - SPI/I2S1,2 and 3 kernel clock source selection"]
pub type SPI123SRC_R = crate::FieldReader;
#[doc = "Field `SPI123SRC` writer - SPI/I2S1,2 and 3 kernel clock source selection"]
pub type SPI123SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI45SRC` reader - SPI4 and 5 kernel clock source selection"]
pub type SPI45SRC_R = crate::FieldReader;
#[doc = "Field `SPI45SRC` writer - SPI4 and 5 kernel clock source selection"]
pub type SPI45SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPDIFSRC` reader - SPDIFRX kernel clock source selection"]
pub type SPDIFSRC_R = crate::FieldReader;
#[doc = "Field `SPDIFSRC` writer - SPDIFRX kernel clock source selection"]
pub type SPDIFSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DFSDM1SRC` reader - DFSDM1 kernel Clk clock source selection"]
pub type DFSDM1SRC_R = crate::BitReader;
#[doc = "Field `DFSDM1SRC` writer - DFSDM1 kernel Clk clock source selection"]
pub type DFSDM1SRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FDCANSRC` reader - FDCAN kernel clock source selection"]
pub type FDCANSRC_R = crate::FieldReader;
#[doc = "Field `FDCANSRC` writer - FDCAN kernel clock source selection"]
pub type FDCANSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SWPSRC` reader - SWPMI kernel clock source selection"]
pub type SWPSRC_R = crate::BitReader;
#[doc = "Field `SWPSRC` writer - SWPMI kernel clock source selection"]
pub type SWPSRC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    pub fn sai1src(&self) -> SAI1SRC_R {
        SAI1SRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    pub fn sai23src(&self) -> SAI23SRC_R {
        SAI23SRC_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    pub fn spi123src(&self) -> SPI123SRC_R {
        SPI123SRC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    pub fn spi45src(&self) -> SPI45SRC_R {
        SPI45SRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifsrc(&self) -> SPDIFSRC_R {
        SPDIFSRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    pub fn dfsdm1src(&self) -> DFSDM1SRC_R {
        DFSDM1SRC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansrc(&self) -> FDCANSRC_R {
        FDCANSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    pub fn swpsrc(&self) -> SWPSRC_R {
        SWPSRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai1src(&mut self) -> SAI1SRC_W<D2CCIP1R_SPEC, 0> {
        SAI1SRC_W::new(self)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai23src(&mut self) -> SAI23SRC_W<D2CCIP1R_SPEC, 6> {
        SAI23SRC_W::new(self)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi123src(&mut self) -> SPI123SRC_W<D2CCIP1R_SPEC, 12> {
        SPI123SRC_W::new(self)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi45src(&mut self) -> SPI45SRC_W<D2CCIP1R_SPEC, 16> {
        SPI45SRC_W::new(self)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spdifsrc(&mut self) -> SPDIFSRC_W<D2CCIP1R_SPEC, 20> {
        SPDIFSRC_W::new(self)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1src(&mut self) -> DFSDM1SRC_W<D2CCIP1R_SPEC, 24> {
        DFSDM1SRC_W::new(self)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn fdcansrc(&mut self) -> FDCANSRC_W<D2CCIP1R_SPEC, 28> {
        FDCANSRC_W::new(self)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn swpsrc(&mut self) -> SWPSRC_W<D2CCIP1R_SPEC, 31> {
        SWPSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2CCIP1R_SPEC;
impl crate::RegisterSpec for D2CCIP1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2ccip1r::R`](R) reader structure"]
impl crate::Readable for D2CCIP1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d2ccip1r::W`](W) writer structure"]
impl crate::Writable for D2CCIP1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D2CCIP1R to value 0"]
impl crate::Resettable for D2CCIP1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
