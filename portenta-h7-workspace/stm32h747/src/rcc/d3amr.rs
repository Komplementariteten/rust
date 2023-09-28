#[doc = "Register `D3AMR` reader"]
pub type R = crate::R<D3AMR_SPEC>;
#[doc = "Register `D3AMR` writer"]
pub type W = crate::W<D3AMR_SPEC>;
#[doc = "Field `BDMAAMEN` reader - BDMA and DMAMUX Autonomous mode enable"]
pub type BDMAAMEN_R = crate::BitReader;
#[doc = "Field `BDMAAMEN` writer - BDMA and DMAMUX Autonomous mode enable"]
pub type BDMAAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable"]
pub type LPUART1AMEN_R = crate::BitReader;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable"]
pub type LPUART1AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable"]
pub type SPI6AMEN_R = crate::BitReader;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable"]
pub type SPI6AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable"]
pub type I2C4AMEN_R = crate::BitReader;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable"]
pub type I2C4AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable"]
pub type LPTIM2AMEN_R = crate::BitReader;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable"]
pub type LPTIM2AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable"]
pub type LPTIM3AMEN_R = crate::BitReader;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable"]
pub type LPTIM3AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM4AMEN` reader - LPTIM4 Autonomous mode enable"]
pub type LPTIM4AMEN_R = crate::BitReader;
#[doc = "Field `LPTIM4AMEN` writer - LPTIM4 Autonomous mode enable"]
pub type LPTIM4AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM5AMEN` reader - LPTIM5 Autonomous mode enable"]
pub type LPTIM5AMEN_R = crate::BitReader;
#[doc = "Field `LPTIM5AMEN` writer - LPTIM5 Autonomous mode enable"]
pub type LPTIM5AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP12AMEN` reader - COMP12 Autonomous mode enable"]
pub type COMP12AMEN_R = crate::BitReader;
#[doc = "Field `COMP12AMEN` writer - COMP12 Autonomous mode enable"]
pub type COMP12AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable"]
pub type VREFAMEN_R = crate::BitReader;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable"]
pub type VREFAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable"]
pub type RTCAMEN_R = crate::BitReader;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable"]
pub type RTCAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCAMEN` reader - CRC Autonomous mode enable"]
pub type CRCAMEN_R = crate::BitReader;
#[doc = "Field `CRCAMEN` writer - CRC Autonomous mode enable"]
pub type CRCAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI4AMEN` reader - SAI4 Autonomous mode enable"]
pub type SAI4AMEN_R = crate::BitReader;
#[doc = "Field `SAI4AMEN` writer - SAI4 Autonomous mode enable"]
pub type SAI4AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3AMEN` reader - ADC3 Autonomous mode enable"]
pub type ADC3AMEN_R = crate::BitReader;
#[doc = "Field `ADC3AMEN` writer - ADC3 Autonomous mode enable"]
pub type ADC3AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKPSRAMAMEN` reader - Backup RAM Autonomous mode enable"]
pub type BKPSRAMAMEN_R = crate::BitReader;
#[doc = "Field `BKPSRAMAMEN` writer - Backup RAM Autonomous mode enable"]
pub type BKPSRAMAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM4AMEN` reader - SRAM4 Autonomous mode enable"]
pub type SRAM4AMEN_R = crate::BitReader;
#[doc = "Field `SRAM4AMEN` writer - SRAM4 Autonomous mode enable"]
pub type SRAM4AMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    pub fn bdmaamen(&self) -> BDMAAMEN_R {
        BDMAAMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim4amen(&self) -> LPTIM4AMEN_R {
        LPTIM4AMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim5amen(&self) -> LPTIM5AMEN_R {
        LPTIM5AMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    pub fn crcamen(&self) -> CRCAMEN_R {
        CRCAMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sai4amen(&self) -> SAI4AMEN_R {
        SAI4AMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    pub fn adc3amen(&self) -> ADC3AMEN_R {
        ADC3AMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    pub fn bkpsramamen(&self) -> BKPSRAMAMEN_R {
        BKPSRAMAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sram4amen(&self) -> SRAM4AMEN_R {
        SRAM4AMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdmaamen(&mut self) -> BDMAAMEN_W<D3AMR_SPEC, 0> {
        BDMAAMEN_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<D3AMR_SPEC, 3> {
        LPUART1AMEN_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W<D3AMR_SPEC, 5> {
        SPI6AMEN_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W<D3AMR_SPEC, 7> {
        I2C4AMEN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W<D3AMR_SPEC, 9> {
        LPTIM2AMEN_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<D3AMR_SPEC, 10> {
        LPTIM3AMEN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4amen(&mut self) -> LPTIM4AMEN_W<D3AMR_SPEC, 11> {
        LPTIM4AMEN_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5amen(&mut self) -> LPTIM5AMEN_W<D3AMR_SPEC, 12> {
        LPTIM5AMEN_W::new(self)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W<D3AMR_SPEC, 14> {
        COMP12AMEN_W::new(self)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<D3AMR_SPEC, 15> {
        VREFAMEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcamen(&mut self) -> RTCAMEN_W<D3AMR_SPEC, 16> {
        RTCAMEN_W::new(self)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcamen(&mut self) -> CRCAMEN_W<D3AMR_SPEC, 19> {
        CRCAMEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai4amen(&mut self) -> SAI4AMEN_W<D3AMR_SPEC, 21> {
        SAI4AMEN_W::new(self)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3amen(&mut self) -> ADC3AMEN_W<D3AMR_SPEC, 24> {
        ADC3AMEN_W::new(self)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramamen(&mut self) -> BKPSRAMAMEN_W<D3AMR_SPEC, 28> {
        BKPSRAMAMEN_W::new(self)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram4amen(&mut self) -> SRAM4AMEN_W<D3AMR_SPEC, 29> {
        SRAM4AMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC D3 Autonomous mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3amr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3amr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3AMR_SPEC;
impl crate::RegisterSpec for D3AMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3amr::R`](R) reader structure"]
impl crate::Readable for D3AMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3amr::W`](W) writer structure"]
impl crate::Writable for D3AMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3AMR to value 0"]
impl crate::Resettable for D3AMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
