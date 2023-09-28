#[doc = "Register `APB4LPENR` reader"]
pub type R = crate::R<APB4LPENR_SPEC>;
#[doc = "Register `APB4LPENR` writer"]
pub type W = crate::W<APB4LPENR_SPEC>;
#[doc = "Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode"]
pub type SYSCFGLPEN_R = crate::BitReader;
#[doc = "Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode"]
pub type SYSCFGLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPUART1LPEN` reader - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub type LPUART1LPEN_R = crate::BitReader;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub type LPUART1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI6LPEN` reader - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI6LPEN_R = crate::BitReader;
#[doc = "Field `SPI6LPEN` writer - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub type SPI6LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C4LPEN` reader - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub type I2C4LPEN_R = crate::BitReader;
#[doc = "Field `I2C4LPEN` writer - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub type I2C4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM2LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM3LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM3LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM4LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM5LPEN_R = crate::BitReader;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub type LPTIM5LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP12LPEN` reader - COMP1/2 peripheral clock enable during CSleep mode"]
pub type COMP12LPEN_R = crate::BitReader;
#[doc = "Field `COMP12LPEN` writer - COMP1/2 peripheral clock enable during CSleep mode"]
pub type COMP12LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode"]
pub type VREFLPEN_R = crate::BitReader;
#[doc = "Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode"]
pub type VREFLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB Clock Enable During CSleep Mode"]
pub type RTCAPBLPEN_R = crate::BitReader;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB Clock Enable During CSleep Mode"]
pub type RTCAPBLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI4LPEN` reader - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI4LPEN_R = crate::BitReader;
#[doc = "Field `SAI4LPEN` writer - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub type SAI4LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<APB4LPENR_SPEC, 1> {
        SYSCFGLPEN_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<APB4LPENR_SPEC, 3> {
        LPUART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<APB4LPENR_SPEC, 5> {
        SPI6LPEN_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<APB4LPENR_SPEC, 7> {
        I2C4LPEN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<APB4LPENR_SPEC, 9> {
        LPTIM2LPEN_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<APB4LPENR_SPEC, 10> {
        LPTIM3LPEN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<APB4LPENR_SPEC, 11> {
        LPTIM4LPEN_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<APB4LPENR_SPEC, 12> {
        LPTIM5LPEN_W::new(self)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W<APB4LPENR_SPEC, 14> {
        COMP12LPEN_W::new(self)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<APB4LPENR_SPEC, 15> {
        VREFLPEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<APB4LPENR_SPEC, 16> {
        RTCAPBLPEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<APB4LPENR_SPEC, 21> {
        SAI4LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB4LPENR_SPEC;
impl crate::RegisterSpec for APB4LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4lpenr::R`](R) reader structure"]
impl crate::Readable for APB4LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb4lpenr::W`](W) writer structure"]
impl crate::Writable for APB4LPENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB4LPENR to value 0"]
impl crate::Resettable for APB4LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
