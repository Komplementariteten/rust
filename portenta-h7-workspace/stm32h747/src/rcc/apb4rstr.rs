#[doc = "Register `APB4RSTR` reader"]
pub type R = crate::R<APB4RSTR_SPEC>;
#[doc = "Register `APB4RSTR` writer"]
pub type W = crate::W<APB4RSTR_SPEC>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG block reset"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG block reset"]
pub type SYSCFGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset"]
pub type LPUART1RST_R = crate::BitReader;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset"]
pub type LPUART1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI6RST` reader - SPI6 block reset"]
pub type SPI6RST_R = crate::BitReader;
#[doc = "Field `SPI6RST` writer - SPI6 block reset"]
pub type SPI6RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C4RST` reader - I2C4 block reset"]
pub type I2C4RST_R = crate::BitReader;
#[doc = "Field `I2C4RST` writer - I2C4 block reset"]
pub type I2C4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset"]
pub type LPTIM2RST_R = crate::BitReader;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset"]
pub type LPTIM2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 block reset"]
pub type LPTIM3RST_R = crate::BitReader;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 block reset"]
pub type LPTIM3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM4RST` reader - LPTIM4 block reset"]
pub type LPTIM4RST_R = crate::BitReader;
#[doc = "Field `LPTIM4RST` writer - LPTIM4 block reset"]
pub type LPTIM4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPTIM5RST` reader - LPTIM5 block reset"]
pub type LPTIM5RST_R = crate::BitReader;
#[doc = "Field `LPTIM5RST` writer - LPTIM5 block reset"]
pub type LPTIM5RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COMP12RST` reader - COMP12 Blocks Reset"]
pub type COMP12RST_R = crate::BitReader;
#[doc = "Field `COMP12RST` writer - COMP12 Blocks Reset"]
pub type COMP12RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFRST` reader - VREF block reset"]
pub type VREFRST_R = crate::BitReader;
#[doc = "Field `VREFRST` writer - VREF block reset"]
pub type VREFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAI4RST` reader - SAI4 block reset"]
pub type SAI4RST_R = crate::BitReader;
#[doc = "Field `SAI4RST` writer - SAI4 block reset"]
pub type SAI4RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - SYSCFG block reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 block reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 block reset"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 block reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 block reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 block reset"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 block reset"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 block reset"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP12 Blocks Reset"]
    #[inline(always)]
    pub fn comp12rst(&self) -> COMP12RST_R {
        COMP12RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF block reset"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 block reset"]
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG block reset"]
    #[inline(always)]
    #[must_use]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<APB4RSTR_SPEC, 1> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB4RSTR_SPEC, 3> {
        LPUART1RST_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn spi6rst(&mut self) -> SPI6RST_W<APB4RSTR_SPEC, 5> {
        SPI6RST_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<APB4RSTR_SPEC, 7> {
        I2C4RST_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APB4RSTR_SPEC, 9> {
        LPTIM2RST_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<APB4RSTR_SPEC, 10> {
        LPTIM3RST_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM4 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<APB4RSTR_SPEC, 11> {
        LPTIM4RST_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM5 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<APB4RSTR_SPEC, 12> {
        LPTIM5RST_W::new(self)
    }
    #[doc = "Bit 14 - COMP12 Blocks Reset"]
    #[inline(always)]
    #[must_use]
    pub fn comp12rst(&mut self) -> COMP12RST_W<APB4RSTR_SPEC, 14> {
        COMP12RST_W::new(self)
    }
    #[doc = "Bit 15 - VREF block reset"]
    #[inline(always)]
    #[must_use]
    pub fn vrefrst(&mut self) -> VREFRST_W<APB4RSTR_SPEC, 15> {
        VREFRST_W::new(self)
    }
    #[doc = "Bit 21 - SAI4 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sai4rst(&mut self) -> SAI4RST_W<APB4RSTR_SPEC, 21> {
        SAI4RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB4RSTR_SPEC;
impl crate::RegisterSpec for APB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4rstr::R`](R) reader structure"]
impl crate::Readable for APB4RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb4rstr::W`](W) writer structure"]
impl crate::Writable for APB4RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB4RSTR to value 0"]
impl crate::Resettable for APB4RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
