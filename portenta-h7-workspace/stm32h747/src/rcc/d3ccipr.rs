#[doc = "Register `D3CCIPR` reader"]
pub type R = crate::R<D3CCIPR_SPEC>;
#[doc = "Register `D3CCIPR` writer"]
pub type W = crate::W<D3CCIPR_SPEC>;
#[doc = "Field `LPUART1SRC` reader - LPUART1 kernel clock source selection"]
pub type LPUART1SRC_R = crate::FieldReader;
#[doc = "Field `LPUART1SRC` writer - LPUART1 kernel clock source selection"]
pub type LPUART1SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `I2C4SRC` reader - I2C4 kernel clock source selection"]
pub type I2C4SRC_R = crate::FieldReader;
#[doc = "Field `I2C4SRC` writer - I2C4 kernel clock source selection"]
pub type I2C4SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPTIM2SRC` reader - LPTIM2 kernel clock source selection"]
pub type LPTIM2SRC_R = crate::FieldReader;
#[doc = "Field `LPTIM2SRC` writer - LPTIM2 kernel clock source selection"]
pub type LPTIM2SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LPTIM345SRC` reader - LPTIM3,4,5 kernel clock source selection"]
pub type LPTIM345SRC_R = crate::FieldReader;
#[doc = "Field `LPTIM345SRC` writer - LPTIM3,4,5 kernel clock source selection"]
pub type LPTIM345SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ADCSRC` reader - SAR ADC kernel clock source selection"]
pub type ADCSRC_R = crate::FieldReader;
#[doc = "Field `ADCSRC` writer - SAR ADC kernel clock source selection"]
pub type ADCSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAI4ASRC` reader - Sub-Block A of SAI4 kernel clock source selection"]
pub type SAI4ASRC_R = crate::FieldReader;
#[doc = "Field `SAI4ASRC` writer - Sub-Block A of SAI4 kernel clock source selection"]
pub type SAI4ASRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SAI4BSRC` reader - Sub-Block B of SAI4 kernel clock source selection"]
pub type SAI4BSRC_R = crate::FieldReader;
#[doc = "Field `SAI4BSRC` writer - Sub-Block B of SAI4 kernel clock source selection"]
pub type SAI4BSRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI6SRC` reader - SPI6 kernel clock source selection"]
pub type SPI6SRC_R = crate::FieldReader;
#[doc = "Field `SPI6SRC` writer - SPI6 kernel clock source selection"]
pub type SPI6SRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    pub fn lpuart1src(&self) -> LPUART1SRC_R {
        LPUART1SRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c4src(&self) -> I2C4SRC_R {
        I2C4SRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim2src(&self) -> LPTIM2SRC_R {
        LPTIM2SRC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim345src(&self) -> LPTIM345SRC_R {
        LPTIM345SRC_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4asrc(&self) -> SAI4ASRC_R {
        SAI4ASRC_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4bsrc(&self) -> SAI4BSRC_R {
        SAI4BSRC_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    pub fn spi6src(&self) -> SPI6SRC_R {
        SPI6SRC_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1src(&mut self) -> LPUART1SRC_W<D3CCIPR_SPEC, 0> {
        LPUART1SRC_W::new(self)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4src(&mut self) -> I2C4SRC_W<D3CCIPR_SPEC, 8> {
        I2C4SRC_W::new(self)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2src(&mut self) -> LPTIM2SRC_W<D3CCIPR_SPEC, 10> {
        LPTIM2SRC_W::new(self)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn lptim345src(&mut self) -> LPTIM345SRC_W<D3CCIPR_SPEC, 13> {
        LPTIM345SRC_W::new(self)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn adcsrc(&mut self) -> ADCSRC_W<D3CCIPR_SPEC, 16> {
        ADCSRC_W::new(self)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai4asrc(&mut self) -> SAI4ASRC_W<D3CCIPR_SPEC, 21> {
        SAI4ASRC_W::new(self)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn sai4bsrc(&mut self) -> SAI4BSRC_W<D3CCIPR_SPEC, 24> {
        SAI4BSRC_W::new(self)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn spi6src(&mut self) -> SPI6SRC_W<D3CCIPR_SPEC, 28> {
        SPI6SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 3 Kernel Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3ccipr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3ccipr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3CCIPR_SPEC;
impl crate::RegisterSpec for D3CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3ccipr::R`](R) reader structure"]
impl crate::Readable for D3CCIPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3ccipr::W`](W) writer structure"]
impl crate::Writable for D3CCIPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3CCIPR to value 0"]
impl crate::Resettable for D3CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
