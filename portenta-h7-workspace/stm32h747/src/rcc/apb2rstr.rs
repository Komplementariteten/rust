#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<APB2RSTR_SPEC>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<APB2RSTR_SPEC>;
#[doc = "Field `TIM1RST` reader - TIM1 block reset"]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 block reset"]
pub type TIM1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM8RST` reader - TIM8 block reset"]
pub type TIM8RST_R = crate::BitReader;
#[doc = "Field `TIM8RST` writer - TIM8 block reset"]
pub type TIM8RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART1RST` reader - USART1 block reset"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 block reset"]
pub type USART1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6RST` reader - USART6 block reset"]
pub type USART6RST_R = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6 block reset"]
pub type USART6RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI1RST` reader - SPI1 block reset"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 block reset"]
pub type SPI1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI4RST` reader - SPI4 block reset"]
pub type SPI4RST_R = crate::BitReader;
#[doc = "Field `SPI4RST` writer - SPI4 block reset"]
pub type SPI4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM15RST` reader - TIM15 block reset"]
pub type TIM15RST_R = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 block reset"]
pub type TIM15RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM16RST` reader - TIM16 block reset"]
pub type TIM16RST_R = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 block reset"]
pub type TIM16RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIM17RST` reader - TIM17 block reset"]
pub type TIM17RST_R = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 block reset"]
pub type TIM17RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI5RST` reader - SPI5 block reset"]
pub type SPI5RST_R = crate::BitReader;
#[doc = "Field `SPI5RST` writer - SPI5 block reset"]
pub type SPI5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI1RST` reader - SAI1 block reset"]
pub type SAI1RST_R = crate::BitReader;
#[doc = "Field `SAI1RST` writer - SAI1 block reset"]
pub type SAI1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI2RST` reader - SAI2 block reset"]
pub type SAI2RST_R = crate::BitReader;
#[doc = "Field `SAI2RST` writer - SAI2 block reset"]
pub type SAI2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI3RST` reader - SAI3 block reset"]
pub type SAI3RST_R = crate::BitReader;
#[doc = "Field `SAI3RST` writer - SAI3 block reset"]
pub type SAI3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DFSDM1RST` reader - DFSDM1 block reset"]
pub type DFSDM1RST_R = crate::BitReader;
#[doc = "Field `DFSDM1RST` writer - DFSDM1 block reset"]
pub type DFSDM1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HRTIMRST` reader - HRTIM block reset"]
pub type HRTIMRST_R = crate::BitReader;
#[doc = "Field `HRTIMRST` writer - HRTIM block reset"]
pub type HRTIMRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TIM1 block reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 block reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 block reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 block reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 block reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 block reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 block reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 block reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 block reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 block reset"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 block reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 block reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 block reset"]
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 block reset"]
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> DFSDM1RST_R {
        DFSDM1RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM block reset"]
    #[inline(always)]
    pub fn hrtimrst(&self) -> HRTIMRST_R {
        HRTIMRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim1rst(&mut self) -> TIM1RST_W<APB2RSTR_SPEC, 0> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim8rst(&mut self) -> TIM8RST_W<APB2RSTR_SPEC, 1> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 4 - USART1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart1rst(&mut self) -> USART1RST_W<APB2RSTR_SPEC, 4> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 5 - USART6 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usart6rst(&mut self) -> USART6RST_W<APB2RSTR_SPEC, 5> {
        USART6RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi1rst(&mut self) -> SPI1RST_W<APB2RSTR_SPEC, 12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi4rst(&mut self) -> SPI4RST_W<APB2RSTR_SPEC, 13> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim15rst(&mut self) -> TIM15RST_W<APB2RSTR_SPEC, 16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim16rst(&mut self) -> TIM16RST_W<APB2RSTR_SPEC, 17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn tim17rst(&mut self) -> TIM17RST_W<APB2RSTR_SPEC, 18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi5rst(&mut self) -> SPI5RST_W<APB2RSTR_SPEC, 20> {
        SPI5RST_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai1rst(&mut self) -> SAI1RST_W<APB2RSTR_SPEC, 22> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 23 - SAI2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai2rst(&mut self) -> SAI2RST_W<APB2RSTR_SPEC, 23> {
        SAI2RST_W::new(self)
    }
    #[doc = "Bit 24 - SAI3 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai3rst(&mut self) -> SAI3RST_W<APB2RSTR_SPEC, 24> {
        SAI3RST_W::new(self)
    }
    #[doc = "Bit 28 - DFSDM1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm1rst(&mut self) -> DFSDM1RST_W<APB2RSTR_SPEC, 28> {
        DFSDM1RST_W::new(self)
    }
    #[doc = "Bit 29 - HRTIM block reset"]
    #[inline(always)]
    #[must_use]
    pub fn hrtimrst(&mut self) -> HRTIMRST_W<APB2RSTR_SPEC, 29> {
        HRTIMRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
