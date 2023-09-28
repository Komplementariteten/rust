#[doc = "Register `AHB4RSTR` reader"]
pub type R = crate::R<AHB4RSTR_SPEC>;
#[doc = "Register `AHB4RSTR` writer"]
pub type W = crate::W<AHB4RSTR_SPEC>;
#[doc = "Field `GPIOARST` reader - GPIO block reset"]
pub type GPIOARST_R = crate::BitReader;
#[doc = "Field `GPIOARST` writer - GPIO block reset"]
pub type GPIOARST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOBRST` reader - GPIO block reset"]
pub type GPIOBRST_R = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - GPIO block reset"]
pub type GPIOBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOCRST` reader - GPIO block reset"]
pub type GPIOCRST_R = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - GPIO block reset"]
pub type GPIOCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIODRST` reader - GPIO block reset"]
pub type GPIODRST_R = crate::BitReader;
#[doc = "Field `GPIODRST` writer - GPIO block reset"]
pub type GPIODRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOERST` reader - GPIO block reset"]
pub type GPIOERST_R = crate::BitReader;
#[doc = "Field `GPIOERST` writer - GPIO block reset"]
pub type GPIOERST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOFRST` reader - GPIO block reset"]
pub type GPIOFRST_R = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - GPIO block reset"]
pub type GPIOFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOGRST` reader - GPIO block reset"]
pub type GPIOGRST_R = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - GPIO block reset"]
pub type GPIOGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOHRST` reader - GPIO block reset"]
pub type GPIOHRST_R = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - GPIO block reset"]
pub type GPIOHRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOIRST` reader - GPIO block reset"]
pub type GPIOIRST_R = crate::BitReader;
#[doc = "Field `GPIOIRST` writer - GPIO block reset"]
pub type GPIOIRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOJRST` reader - GPIO block reset"]
pub type GPIOJRST_R = crate::BitReader;
#[doc = "Field `GPIOJRST` writer - GPIO block reset"]
pub type GPIOJRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOKRST` reader - GPIO block reset"]
pub type GPIOKRST_R = crate::BitReader;
#[doc = "Field `GPIOKRST` writer - GPIO block reset"]
pub type GPIOKRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCRST` reader - CRC block reset"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC block reset"]
pub type CRCRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BDMARST` reader - BDMA block reset"]
pub type BDMARST_R = crate::BitReader;
#[doc = "Field `BDMARST` writer - BDMA block reset"]
pub type BDMARST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3RST` reader - ADC3 block reset"]
pub type ADC3RST_R = crate::BitReader;
#[doc = "Field `ADC3RST` writer - ADC3 block reset"]
pub type ADC3RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEMRST` reader - HSEM block reset"]
pub type HSEMRST_R = crate::BitReader;
#[doc = "Field `HSEMRST` writer - HSEM block reset"]
pub type HSEMRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC block reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA block reset"]
    #[inline(always)]
    pub fn bdmarst(&self) -> BDMARST_R {
        BDMARST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 block reset"]
    #[inline(always)]
    pub fn adc3rst(&self) -> ADC3RST_R {
        ADC3RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM block reset"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HSEMRST_R {
        HSEMRST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<AHB4RSTR_SPEC, 0> {
        GPIOARST_W::new(self)
    }
    #[doc = "Bit 1 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<AHB4RSTR_SPEC, 1> {
        GPIOBRST_W::new(self)
    }
    #[doc = "Bit 2 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<AHB4RSTR_SPEC, 2> {
        GPIOCRST_W::new(self)
    }
    #[doc = "Bit 3 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<AHB4RSTR_SPEC, 3> {
        GPIODRST_W::new(self)
    }
    #[doc = "Bit 4 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<AHB4RSTR_SPEC, 4> {
        GPIOERST_W::new(self)
    }
    #[doc = "Bit 5 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<AHB4RSTR_SPEC, 5> {
        GPIOFRST_W::new(self)
    }
    #[doc = "Bit 6 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<AHB4RSTR_SPEC, 6> {
        GPIOGRST_W::new(self)
    }
    #[doc = "Bit 7 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<AHB4RSTR_SPEC, 7> {
        GPIOHRST_W::new(self)
    }
    #[doc = "Bit 8 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<AHB4RSTR_SPEC, 8> {
        GPIOIRST_W::new(self)
    }
    #[doc = "Bit 9 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<AHB4RSTR_SPEC, 9> {
        GPIOJRST_W::new(self)
    }
    #[doc = "Bit 10 - GPIO block reset"]
    #[inline(always)]
    #[must_use]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<AHB4RSTR_SPEC, 10> {
        GPIOKRST_W::new(self)
    }
    #[doc = "Bit 19 - CRC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<AHB4RSTR_SPEC, 19> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 21 - BDMA block reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdmarst(&mut self) -> BDMARST_W<AHB4RSTR_SPEC, 21> {
        BDMARST_W::new(self)
    }
    #[doc = "Bit 24 - ADC3 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc3rst(&mut self) -> ADC3RST_W<AHB4RSTR_SPEC, 24> {
        ADC3RST_W::new(self)
    }
    #[doc = "Bit 25 - HSEM block reset"]
    #[inline(always)]
    #[must_use]
    pub fn hsemrst(&mut self) -> HSEMRST_W<AHB4RSTR_SPEC, 25> {
        HSEMRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB4 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4RSTR_SPEC;
impl crate::RegisterSpec for AHB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4rstr::R`](R) reader structure"]
impl crate::Readable for AHB4RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure"]
impl crate::Writable for AHB4RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for AHB4RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
