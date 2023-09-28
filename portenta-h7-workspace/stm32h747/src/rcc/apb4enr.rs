#[doc = "Register `APB4ENR` reader"]
pub type R = crate::R<APB4ENR_SPEC>;
#[doc = "Register `APB4ENR` writer"]
pub type W = crate::W<APB4ENR_SPEC>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG peripheral clock enable"]
pub type SYSCFGEN_R = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG peripheral clock enable"]
pub type SYSCFGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPUART1EN` reader - LPUART1 Peripheral Clocks Enable"]
pub type LPUART1EN_R = crate::BitReader;
#[doc = "Field `LPUART1EN` writer - LPUART1 Peripheral Clocks Enable"]
pub type LPUART1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI6EN` reader - SPI6 Peripheral Clocks Enable"]
pub type SPI6EN_R = crate::BitReader;
#[doc = "Field `SPI6EN` writer - SPI6 Peripheral Clocks Enable"]
pub type SPI6EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C4EN` reader - I2C4 Peripheral Clocks Enable"]
pub type I2C4EN_R = crate::BitReader;
#[doc = "Field `I2C4EN` writer - I2C4 Peripheral Clocks Enable"]
pub type I2C4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 Peripheral Clocks Enable"]
pub type LPTIM2EN_R = crate::BitReader;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 Peripheral Clocks Enable"]
pub type LPTIM2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 Peripheral Clocks Enable"]
pub type LPTIM3EN_R = crate::BitReader;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 Peripheral Clocks Enable"]
pub type LPTIM3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 Peripheral Clocks Enable"]
pub type LPTIM4EN_R = crate::BitReader;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 Peripheral Clocks Enable"]
pub type LPTIM4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM5EN` reader - LPTIM5 Peripheral Clocks Enable"]
pub type LPTIM5EN_R = crate::BitReader;
#[doc = "Field `LPTIM5EN` writer - LPTIM5 Peripheral Clocks Enable"]
pub type LPTIM5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP12EN` reader - COMP1/2 peripheral clock enable"]
pub type COMP12EN_R = crate::BitReader;
#[doc = "Field `COMP12EN` writer - COMP1/2 peripheral clock enable"]
pub type COMP12EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFEN` reader - VREF peripheral clock enable"]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREF peripheral clock enable"]
pub type VREFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTCAPBEN` reader - RTC APB Clock Enable"]
pub type RTCAPBEN_R = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB Clock Enable"]
pub type RTCAPBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI4EN` reader - SAI4 Peripheral Clocks Enable"]
pub type SAI4EN_R = crate::BitReader;
#[doc = "Field `SAI4EN` writer - SAI4 Peripheral Clocks Enable"]
pub type SAI4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    pub fn comp12en(&self) -> COMP12EN_R {
        COMP12EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<APB4ENR_SPEC, 1> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<APB4ENR_SPEC, 3> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi6en(&mut self) -> SPI6EN_W<APB4ENR_SPEC, 5> {
        SPI6EN_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<APB4ENR_SPEC, 7> {
        I2C4EN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APB4ENR_SPEC, 9> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<APB4ENR_SPEC, 10> {
        LPTIM3EN_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<APB4ENR_SPEC, 11> {
        LPTIM4EN_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W<APB4ENR_SPEC, 12> {
        LPTIM5EN_W::new(self)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp12en(&mut self) -> COMP12EN_W<APB4ENR_SPEC, 14> {
        COMP12EN_W::new(self)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<APB4ENR_SPEC, 15> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<APB4ENR_SPEC, 16> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai4en(&mut self) -> SAI4EN_W<APB4ENR_SPEC, 21> {
        SAI4EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB4ENR_SPEC;
impl crate::RegisterSpec for APB4ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4enr::R`](R) reader structure"]
impl crate::Readable for APB4ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb4enr::W`](W) writer structure"]
impl crate::Writable for APB4ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB4ENR to value 0"]
impl crate::Resettable for APB4ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
