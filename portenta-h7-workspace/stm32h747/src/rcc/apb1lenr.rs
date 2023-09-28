#[doc = "Register `APB1LENR` reader"]
pub type R = crate::R<APB1LENR_SPEC>;
#[doc = "Register `APB1LENR` writer"]
pub type W = crate::W<APB1LENR_SPEC>;
#[doc = "Field `TIM2EN` reader - TIM peripheral clock enable"]
pub type TIM2EN_R = crate::BitReader;
#[doc = "Field `TIM2EN` writer - TIM peripheral clock enable"]
pub type TIM2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM3EN` reader - TIM peripheral clock enable"]
pub type TIM3EN_R = crate::BitReader;
#[doc = "Field `TIM3EN` writer - TIM peripheral clock enable"]
pub type TIM3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM4EN` reader - TIM peripheral clock enable"]
pub type TIM4EN_R = crate::BitReader;
#[doc = "Field `TIM4EN` writer - TIM peripheral clock enable"]
pub type TIM4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM5EN` reader - TIM peripheral clock enable"]
pub type TIM5EN_R = crate::BitReader;
#[doc = "Field `TIM5EN` writer - TIM peripheral clock enable"]
pub type TIM5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM6EN` reader - TIM peripheral clock enable"]
pub type TIM6EN_R = crate::BitReader;
#[doc = "Field `TIM6EN` writer - TIM peripheral clock enable"]
pub type TIM6EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM7EN` reader - TIM peripheral clock enable"]
pub type TIM7EN_R = crate::BitReader;
#[doc = "Field `TIM7EN` writer - TIM peripheral clock enable"]
pub type TIM7EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM12EN` reader - TIM peripheral clock enable"]
pub type TIM12EN_R = crate::BitReader;
#[doc = "Field `TIM12EN` writer - TIM peripheral clock enable"]
pub type TIM12EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM13EN` reader - TIM peripheral clock enable"]
pub type TIM13EN_R = crate::BitReader;
#[doc = "Field `TIM13EN` writer - TIM peripheral clock enable"]
pub type TIM13EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM14EN` reader - TIM peripheral clock enable"]
pub type TIM14EN_R = crate::BitReader;
#[doc = "Field `TIM14EN` writer - TIM peripheral clock enable"]
pub type TIM14EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 Peripheral Clocks Enable"]
pub type LPTIM1EN_R = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 Peripheral Clocks Enable"]
pub type LPTIM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI2EN` reader - SPI2 Peripheral Clocks Enable"]
pub type SPI2EN_R = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 Peripheral Clocks Enable"]
pub type SPI2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI3EN` reader - SPI3 Peripheral Clocks Enable"]
pub type SPI3EN_R = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI3 Peripheral Clocks Enable"]
pub type SPI3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPDIFRXEN` reader - SPDIFRX Peripheral Clocks Enable"]
pub type SPDIFRXEN_R = crate::BitReader;
#[doc = "Field `SPDIFRXEN` writer - SPDIFRX Peripheral Clocks Enable"]
pub type SPDIFRXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART2EN` reader - USART2 Peripheral Clocks Enable"]
pub type USART2EN_R = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 Peripheral Clocks Enable"]
pub type USART2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART3EN` reader - USART3 Peripheral Clocks Enable"]
pub type USART3EN_R = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART3 Peripheral Clocks Enable"]
pub type USART3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART4EN` reader - UART4 Peripheral Clocks Enable"]
pub type UART4EN_R = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART4 Peripheral Clocks Enable"]
pub type UART4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART5EN` reader - UART5 Peripheral Clocks Enable"]
pub type UART5EN_R = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART5 Peripheral Clocks Enable"]
pub type UART5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1EN` reader - I2C1 Peripheral Clocks Enable"]
pub type I2C1EN_R = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 Peripheral Clocks Enable"]
pub type I2C1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C2EN` reader - I2C2 Peripheral Clocks Enable"]
pub type I2C2EN_R = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 Peripheral Clocks Enable"]
pub type I2C2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C3EN` reader - I2C3 Peripheral Clocks Enable"]
pub type I2C3EN_R = crate::BitReader;
#[doc = "Field `I2C3EN` writer - I2C3 Peripheral Clocks Enable"]
pub type I2C3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HDMICECEN` reader - HDMI-CEC peripheral clock enable"]
pub type HDMICECEN_R = crate::BitReader;
#[doc = "Field `HDMICECEN` writer - HDMI-CEC peripheral clock enable"]
pub type HDMICECEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DAC12EN` reader - DAC1&amp;2 peripheral clock enable"]
pub type DAC12EN_R = crate::BitReader;
#[doc = "Field `DAC12EN` writer - DAC1&amp;2 peripheral clock enable"]
pub type DAC12EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART7EN` reader - USART7 Peripheral Clocks Enable"]
pub type USART7EN_R = crate::BitReader;
#[doc = "Field `USART7EN` writer - USART7 Peripheral Clocks Enable"]
pub type USART7EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART8EN` reader - USART8 Peripheral Clocks Enable"]
pub type USART8EN_R = crate::BitReader;
#[doc = "Field `USART8EN` writer - USART8 Peripheral Clocks Enable"]
pub type USART8EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEC peripheral clock enable"]
    #[inline(always)]
    pub fn hdmicecen(&self) -> HDMICECEN_R {
        HDMICECEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1&amp;2 peripheral clock enable"]
    #[inline(always)]
    pub fn dac12en(&self) -> DAC12EN_R {
        DAC12EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USART7 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart7en(&self) -> USART7EN_R {
        USART7EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USART8 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart8en(&self) -> USART8EN_R {
        USART8EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim2en(&mut self) -> TIM2EN_W<APB1LENR_SPEC, 0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim3en(&mut self) -> TIM3EN_W<APB1LENR_SPEC, 1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim4en(&mut self) -> TIM4EN_W<APB1LENR_SPEC, 2> {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim5en(&mut self) -> TIM5EN_W<APB1LENR_SPEC, 3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim6en(&mut self) -> TIM6EN_W<APB1LENR_SPEC, 4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim7en(&mut self) -> TIM7EN_W<APB1LENR_SPEC, 5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 6 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim12en(&mut self) -> TIM12EN_W<APB1LENR_SPEC, 6> {
        TIM12EN_W::new(self)
    }
    #[doc = "Bit 7 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim13en(&mut self) -> TIM13EN_W<APB1LENR_SPEC, 7> {
        TIM13EN_W::new(self)
    }
    #[doc = "Bit 8 - TIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim14en(&mut self) -> TIM14EN_W<APB1LENR_SPEC, 8> {
        TIM14EN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<APB1LENR_SPEC, 9> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi2en(&mut self) -> SPI2EN_W<APB1LENR_SPEC, 14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<APB1LENR_SPEC, 15> {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<APB1LENR_SPEC, 16> {
        SPDIFRXEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2en(&mut self) -> USART2EN_W<APB1LENR_SPEC, 17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3en(&mut self) -> USART3EN_W<APB1LENR_SPEC, 18> {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4en(&mut self) -> UART4EN_W<APB1LENR_SPEC, 19> {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart5en(&mut self) -> UART5EN_W<APB1LENR_SPEC, 20> {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1en(&mut self) -> I2C1EN_W<APB1LENR_SPEC, 21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2en(&mut self) -> I2C2EN_W<APB1LENR_SPEC, 22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<APB1LENR_SPEC, 23> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 27 - HDMI-CEC peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdmicecen(&mut self) -> HDMICECEN_W<APB1LENR_SPEC, 27> {
        HDMICECEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC1&amp;2 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac12en(&mut self) -> DAC12EN_W<APB1LENR_SPEC, 29> {
        DAC12EN_W::new(self)
    }
    #[doc = "Bit 30 - USART7 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart7en(&mut self) -> USART7EN_W<APB1LENR_SPEC, 30> {
        USART7EN_W::new(self)
    }
    #[doc = "Bit 31 - USART8 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart8en(&mut self) -> USART8EN_W<APB1LENR_SPEC, 31> {
        USART8EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LENR_SPEC;
impl crate::RegisterSpec for APB1LENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lenr::R`](R) reader structure"]
impl crate::Readable for APB1LENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb1lenr::W`](W) writer structure"]
impl crate::Writable for APB1LENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LENR to value 0"]
impl crate::Resettable for APB1LENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
