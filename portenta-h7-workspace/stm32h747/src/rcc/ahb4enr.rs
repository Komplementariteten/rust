#[doc = "Register `AHB4ENR` reader"]
pub type R = crate::R<AHB4ENR_SPEC>;
#[doc = "Register `AHB4ENR` writer"]
pub type W = crate::W<AHB4ENR_SPEC>;
#[doc = "Field `GPIOAEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOBEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOCEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIODEN` reader - 0GPIO peripheral clock enable"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - 0GPIO peripheral clock enable"]
pub type GPIODEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOEEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOFEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOGEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOGEN_R = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOHEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOHEN_R = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOHEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOIEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOIEN_R = crate::BitReader;
#[doc = "Field `GPIOIEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOJEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOJEN_R = crate::BitReader;
#[doc = "Field `GPIOJEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOJEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPIOKEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOKEN_R = crate::BitReader;
#[doc = "Field `GPIOKEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRCEN` reader - CRC peripheral clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC peripheral clock enable"]
pub type CRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BDMAEN` reader - BDMA and DMAMUX2 Clock Enable"]
pub type BDMAEN_R = crate::BitReader;
#[doc = "Field `BDMAEN` writer - BDMA and DMAMUX2 Clock Enable"]
pub type BDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC3EN` reader - ADC3 Peripheral Clocks Enable"]
pub type ADC3EN_R = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 Peripheral Clocks Enable"]
pub type ADC3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEMEN` reader - HSEM peripheral clock enable"]
pub type HSEMEN_R = crate::BitReader;
#[doc = "Field `HSEMEN` writer - HSEM peripheral clock enable"]
pub type HSEMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BKPRAMEN` reader - Backup RAM Clock Enable"]
pub type BKPRAMEN_R = crate::BitReader;
#[doc = "Field `BKPRAMEN` writer - Backup RAM Clock Enable"]
pub type BKPRAMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    pub fn bdmaen(&self) -> BDMAEN_R {
        BDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<AHB4ENR_SPEC, 0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<AHB4ENR_SPEC, 1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<AHB4ENR_SPEC, 2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<AHB4ENR_SPEC, 3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<AHB4ENR_SPEC, 4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<AHB4ENR_SPEC, 5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<AHB4ENR_SPEC, 6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<AHB4ENR_SPEC, 7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<AHB4ENR_SPEC, 8> {
        GPIOIEN_W::new(self)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<AHB4ENR_SPEC, 9> {
        GPIOJEN_W::new(self)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpioken(&mut self) -> GPIOKEN_W<AHB4ENR_SPEC, 10> {
        GPIOKEN_W::new(self)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHB4ENR_SPEC, 19> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdmaen(&mut self) -> BDMAEN_W<AHB4ENR_SPEC, 21> {
        BDMAEN_W::new(self)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc3en(&mut self) -> ADC3EN_W<AHB4ENR_SPEC, 24> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<AHB4ENR_SPEC, 25> {
        HSEMEN_W::new(self)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<AHB4ENR_SPEC, 28> {
        BKPRAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB4 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb4enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb4enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB4ENR_SPEC;
impl crate::RegisterSpec for AHB4ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4enr::R`](R) reader structure"]
impl crate::Readable for AHB4ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb4enr::W`](W) writer structure"]
impl crate::Writable for AHB4ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB4ENR to value 0"]
impl crate::Resettable for AHB4ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
