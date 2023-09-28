#[doc = "Register `APB2LPENR` reader"]
pub type R = crate::R<APB2LPENR_SPEC>;
#[doc = "Register `APB2LPENR` writer"]
pub type W = crate::W<APB2LPENR_SPEC>;
#[doc = "Field `TIM1LPEN` reader - TIM1 peripheral clock enable during CSleep mode"]
pub type TIM1LPEN_R = crate::BitReader;
#[doc = "Field `TIM1LPEN` writer - TIM1 peripheral clock enable during CSleep mode"]
pub type TIM1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM8LPEN` reader - TIM8 peripheral clock enable during CSleep mode"]
pub type TIM8LPEN_R = crate::BitReader;
#[doc = "Field `TIM8LPEN` writer - TIM8 peripheral clock enable during CSleep mode"]
pub type TIM8LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1LPEN` reader - USART1 Peripheral Clocks Enable During CSleep Mode"]
pub type USART1LPEN_R = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 Peripheral Clocks Enable During CSleep Mode"]
pub type USART1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6LPEN` reader - USART6 Peripheral Clocks Enable During CSleep Mode"]
pub type USART6LPEN_R = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 Peripheral Clocks Enable During CSleep Mode"]
pub type USART6LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1LPEN` reader - SPI1 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI1LPEN_R = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4LPEN` reader - SPI4 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI4LPEN_R = crate::BitReader;
#[doc = "Field `SPI4LPEN` writer - SPI4 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM15LPEN` reader - TIM15 peripheral clock enable during CSleep mode"]
pub type TIM15LPEN_R = crate::BitReader;
#[doc = "Field `TIM15LPEN` writer - TIM15 peripheral clock enable during CSleep mode"]
pub type TIM15LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM16LPEN` reader - TIM16 peripheral clock enable during CSleep mode"]
pub type TIM16LPEN_R = crate::BitReader;
#[doc = "Field `TIM16LPEN` writer - TIM16 peripheral clock enable during CSleep mode"]
pub type TIM16LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM17LPEN` reader - TIM17 peripheral clock enable during CSleep mode"]
pub type TIM17LPEN_R = crate::BitReader;
#[doc = "Field `TIM17LPEN` writer - TIM17 peripheral clock enable during CSleep mode"]
pub type TIM17LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI5LPEN` reader - SPI5 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI5LPEN_R = crate::BitReader;
#[doc = "Field `SPI5LPEN` writer - SPI5 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI5LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI1LPEN` reader - SAI1 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI1LPEN_R = crate::BitReader;
#[doc = "Field `SAI1LPEN` writer - SAI1 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI2LPEN` reader - SAI2 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI2LPEN_R = crate::BitReader;
#[doc = "Field `SAI2LPEN` writer - SAI2 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI3LPEN` reader - SAI3 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI3LPEN_R = crate::BitReader;
#[doc = "Field `SAI3LPEN` writer - SAI3 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI3LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFSDM1LPEN` reader - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
pub type DFSDM1LPEN_R = crate::BitReader;
#[doc = "Field `DFSDM1LPEN` writer - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
pub type DFSDM1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRTIMLPEN` reader - HRTIM peripheral clock enable during CSleep mode"]
pub type HRTIMLPEN_R = crate::BitReader;
#[doc = "Field `HRTIMLPEN` writer - HRTIM peripheral clock enable during CSleep mode"]
pub type HRTIMLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> DFSDM1LPEN_R {
        DFSDM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hrtimlpen(&self) -> HRTIMLPEN_R {
        HRTIMLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<APB2LPENR_SPEC, 0> {
        TIM1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<APB2LPENR_SPEC, 1> {
        TIM8LPEN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<APB2LPENR_SPEC, 4> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<APB2LPENR_SPEC, 5> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<APB2LPENR_SPEC, 12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<APB2LPENR_SPEC, 13> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<APB2LPENR_SPEC, 16> {
        TIM15LPEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<APB2LPENR_SPEC, 17> {
        TIM16LPEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<APB2LPENR_SPEC, 18> {
        TIM17LPEN_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<APB2LPENR_SPEC, 20> {
        SPI5LPEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<APB2LPENR_SPEC, 22> {
        SAI1LPEN_W::new(self)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<APB2LPENR_SPEC, 23> {
        SAI2LPEN_W::new(self)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W<APB2LPENR_SPEC, 24> {
        SAI3LPEN_W::new(self)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1lpen(&mut self) -> DFSDM1LPEN_W<APB2LPENR_SPEC, 28> {
        DFSDM1LPEN_W::new(self)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrtimlpen(&mut self) -> HRTIMLPEN_W<APB2LPENR_SPEC, 29> {
        HRTIMLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpenr::R`](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2LPENR to value 0"]
impl crate::Resettable for APB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
