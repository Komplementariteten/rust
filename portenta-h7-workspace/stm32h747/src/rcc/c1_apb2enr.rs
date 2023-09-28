#[doc = "Register `C1_APB2ENR` reader"]
pub type R = crate::R<C1_APB2ENR_SPEC>;
#[doc = "Register `C1_APB2ENR` writer"]
pub type W = crate::W<C1_APB2ENR_SPEC>;
#[doc = "Field `TIM1EN` reader - TIM1 peripheral clock enable"]
pub type TIM1EN_R = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 peripheral clock enable"]
pub type TIM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM8EN` reader - TIM8 peripheral clock enable"]
pub type TIM8EN_R = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8 peripheral clock enable"]
pub type TIM8EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1EN` reader - USART1 Peripheral Clocks Enable"]
pub type USART1EN_R = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 Peripheral Clocks Enable"]
pub type USART1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6EN` reader - USART6 Peripheral Clocks Enable"]
pub type USART6EN_R = crate::BitReader;
#[doc = "Field `USART6EN` writer - USART6 Peripheral Clocks Enable"]
pub type USART6EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1EN` reader - SPI1 Peripheral Clocks Enable"]
pub type SPI1EN_R = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 Peripheral Clocks Enable"]
pub type SPI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4EN` reader - SPI4 Peripheral Clocks Enable"]
pub type SPI4EN_R = crate::BitReader;
#[doc = "Field `SPI4EN` writer - SPI4 Peripheral Clocks Enable"]
pub type SPI4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM15EN` reader - TIM15 peripheral clock enable"]
pub type TIM15EN_R = crate::BitReader;
#[doc = "Field `TIM15EN` writer - TIM15 peripheral clock enable"]
pub type TIM15EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM16EN` reader - TIM16 peripheral clock enable"]
pub type TIM16EN_R = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 peripheral clock enable"]
pub type TIM16EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM17EN` reader - TIM17 peripheral clock enable"]
pub type TIM17EN_R = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM17 peripheral clock enable"]
pub type TIM17EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI5EN` reader - SPI5 Peripheral Clocks Enable"]
pub type SPI5EN_R = crate::BitReader;
#[doc = "Field `SPI5EN` writer - SPI5 Peripheral Clocks Enable"]
pub type SPI5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI1EN` reader - SAI1 Peripheral Clocks Enable"]
pub type SAI1EN_R = crate::BitReader;
#[doc = "Field `SAI1EN` writer - SAI1 Peripheral Clocks Enable"]
pub type SAI1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI2EN` reader - SAI2 Peripheral Clocks Enable"]
pub type SAI2EN_R = crate::BitReader;
#[doc = "Field `SAI2EN` writer - SAI2 Peripheral Clocks Enable"]
pub type SAI2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI3EN` reader - SAI3 Peripheral Clocks Enable"]
pub type SAI3EN_R = crate::BitReader;
#[doc = "Field `SAI3EN` writer - SAI3 Peripheral Clocks Enable"]
pub type SAI3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFSDM1EN` reader - DFSDM1 Peripheral Clocks Enable"]
pub type DFSDM1EN_R = crate::BitReader;
#[doc = "Field `DFSDM1EN` writer - DFSDM1 Peripheral Clocks Enable"]
pub type DFSDM1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRTIMEN` reader - HRTIM peripheral clock enable"]
pub type HRTIMEN_R = crate::BitReader;
#[doc = "Field `HRTIMEN` writer - HRTIM peripheral clock enable"]
pub type HRTIMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn dfsdm1en(&self) -> DFSDM1EN_R {
        DFSDM1EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable"]
    #[inline(always)]
    pub fn hrtimen(&self) -> HRTIMEN_R {
        HRTIMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<C1_APB2ENR_SPEC, 0> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim8en(&mut self) -> TIM8EN_W<C1_APB2ENR_SPEC, 1> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<C1_APB2ENR_SPEC, 4> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart6en(&mut self) -> USART6EN_W<C1_APB2ENR_SPEC, 5> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<C1_APB2ENR_SPEC, 12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi4en(&mut self) -> SPI4EN_W<C1_APB2ENR_SPEC, 13> {
        SPI4EN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim15en(&mut self) -> TIM15EN_W<C1_APB2ENR_SPEC, 16> {
        TIM15EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<C1_APB2ENR_SPEC, 17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<C1_APB2ENR_SPEC, 18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi5en(&mut self) -> SPI5EN_W<C1_APB2ENR_SPEC, 20> {
        SPI5EN_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<C1_APB2ENR_SPEC, 22> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai2en(&mut self) -> SAI2EN_W<C1_APB2ENR_SPEC, 23> {
        SAI2EN_W::new(self)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sai3en(&mut self) -> SAI3EN_W<C1_APB2ENR_SPEC, 24> {
        SAI3EN_W::new(self)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1en(&mut self) -> DFSDM1EN_W<C1_APB2ENR_SPEC, 28> {
        DFSDM1EN_W::new(self)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrtimen(&mut self) -> HRTIMEN_W<C1_APB2ENR_SPEC, 29> {
        HRTIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB2 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb2enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb2enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_APB2ENR_SPEC;
impl crate::RegisterSpec for C1_APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb2enr::R`](R) reader structure"]
impl crate::Readable for C1_APB2ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_apb2enr::W`](W) writer structure"]
impl crate::Writable for C1_APB2ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_APB2ENR to value 0"]
impl crate::Resettable for C1_APB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
