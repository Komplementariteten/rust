#[doc = "Register `D2CCIP2R` reader"]
pub type R = crate::R<D2CCIP2R_SPEC>;
#[doc = "Register `D2CCIP2R` writer"]
pub type W = crate::W<D2CCIP2R_SPEC>;
#[doc = "Field `USART234578SRC` reader - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
pub type USART234578SRC_R = crate::FieldReader;
#[doc = "Field `USART234578SRC` writer - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
pub type USART234578SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `USART16SRC` reader - USART1 and 6 kernel clock source selection"]
pub type USART16SRC_R = crate::FieldReader;
#[doc = "Field `USART16SRC` writer - USART1 and 6 kernel clock source selection"]
pub type USART16SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RNGSRC` reader - RNG kernel clock source selection"]
pub type RNGSRC_R = crate::FieldReader;
#[doc = "Field `RNGSRC` writer - RNG kernel clock source selection"]
pub type RNGSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `I2C123SRC` reader - I2C1,2,3 kernel clock source selection"]
pub type I2C123SRC_R = crate::FieldReader;
#[doc = "Field `I2C123SRC` writer - I2C1,2,3 kernel clock source selection"]
pub type I2C123SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `USBSRC` reader - USBOTG 1 and 2 kernel clock source selection"]
pub type USBSRC_R = crate::FieldReader;
#[doc = "Field `USBSRC` writer - USBOTG 1 and 2 kernel clock source selection"]
pub type USBSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CECSRC` reader - HDMI-CEC kernel clock source selection"]
pub type CECSRC_R = crate::FieldReader;
#[doc = "Field `CECSRC` writer - HDMI-CEC kernel clock source selection"]
pub type CECSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPTIM1SRC` reader - LPTIM1 kernel clock source selection"]
pub type LPTIM1SRC_R = crate::FieldReader;
#[doc = "Field `LPTIM1SRC` writer - LPTIM1 kernel clock source selection"]
pub type LPTIM1SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    pub fn usart234578src(&self) -> USART234578SRC_R {
        USART234578SRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART1 and 6 kernel clock source selection"]
    #[inline(always)]
    pub fn usart16src(&self) -> USART16SRC_R {
        USART16SRC_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsrc(&self) -> RNGSRC_R {
        RNGSRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c123src(&self) -> I2C123SRC_R {
        I2C123SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection"]
    #[inline(always)]
    pub fn usbsrc(&self) -> USBSRC_R {
        USBSRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection"]
    #[inline(always)]
    pub fn cecsrc(&self) -> CECSRC_R {
        CECSRC_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1src(&self) -> LPTIM1SRC_R {
        LPTIM1SRC_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart234578src(&mut self) -> USART234578SRC_W<D2CCIP2R_SPEC, 0> {
        USART234578SRC_W::new(self)
    }
    #[doc = "Bits 3:5 - USART1 and 6 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usart16src(&mut self) -> USART16SRC_W<D2CCIP2R_SPEC, 3> {
        USART16SRC_W::new(self)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rngsrc(&mut self) -> RNGSRC_W<D2CCIP2R_SPEC, 8> {
        RNGSRC_W::new(self)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c123src(&mut self) -> I2C123SRC_W<D2CCIP2R_SPEC, 12> {
        I2C123SRC_W::new(self)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn usbsrc(&mut self) -> USBSRC_W<D2CCIP2R_SPEC, 20> {
        USBSRC_W::new(self)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn cecsrc(&mut self) -> CECSRC_W<D2CCIP2R_SPEC, 22> {
        CECSRC_W::new(self)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim1src(&mut self) -> LPTIM1SRC_W<D2CCIP2R_SPEC, 28> {
        LPTIM1SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2ccip2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2ccip2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2CCIP2R_SPEC;
impl crate::RegisterSpec for D2CCIP2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2ccip2r::R`](R) reader structure"]
impl crate::Readable for D2CCIP2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d2ccip2r::W`](W) writer structure"]
impl crate::Writable for D2CCIP2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D2CCIP2R to value 0"]
impl crate::Resettable for D2CCIP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
