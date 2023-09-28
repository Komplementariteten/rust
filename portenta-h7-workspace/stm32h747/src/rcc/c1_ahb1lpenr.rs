#[doc = "Register `C1_AHB1LPENR` reader"]
pub type R = crate::R<C1_AHB1LPENR_SPEC>;
#[doc = "Register `C1_AHB1LPENR` writer"]
pub type W = crate::W<C1_AHB1LPENR_SPEC>;
#[doc = "Field `DMA1LPEN` reader - DMA1 Clock Enable During CSleep Mode"]
pub type DMA1LPEN_R = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 Clock Enable During CSleep Mode"]
pub type DMA1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2LPEN` reader - DMA2 Clock Enable During CSleep Mode"]
pub type DMA2LPEN_R = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 Clock Enable During CSleep Mode"]
pub type DMA2LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC12LPEN` reader - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type ADC12LPEN_R = crate::BitReader;
#[doc = "Field `ADC12LPEN` writer - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type ADC12LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1MACLPEN` reader - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type ETH1MACLPEN_R = crate::BitReader;
#[doc = "Field `ETH1MACLPEN` writer - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type ETH1MACLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1TXLPEN` reader - Ethernet Transmission Clock Enable During CSleep Mode"]
pub type ETH1TXLPEN_R = crate::BitReader;
#[doc = "Field `ETH1TXLPEN` writer - Ethernet Transmission Clock Enable During CSleep Mode"]
pub type ETH1TXLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1RXLPEN` reader - Ethernet Reception Clock Enable During CSleep Mode"]
pub type ETH1RXLPEN_R = crate::BitReader;
#[doc = "Field `ETH1RXLPEN` writer - Ethernet Reception Clock Enable During CSleep Mode"]
pub type ETH1RXLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB1OTGLPEN` reader - USB1OTG peripheral clock enable during CSleep mode"]
pub type USB1OTGLPEN_R = crate::BitReader;
#[doc = "Field `USB1OTGLPEN` writer - USB1OTG peripheral clock enable during CSleep mode"]
pub type USB1OTGLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB1ULPILPEN` reader - USB_PHY1 clock enable during CSleep mode"]
pub type USB1ULPILPEN_R = crate::BitReader;
#[doc = "Field `USB1ULPILPEN` writer - USB_PHY1 clock enable during CSleep mode"]
pub type USB1ULPILPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB2OTGLPEN` reader - USB2OTG peripheral clock enable during CSleep mode"]
pub type USB2OTGLPEN_R = crate::BitReader;
#[doc = "Field `USB2OTGLPEN` writer - USB2OTG peripheral clock enable during CSleep mode"]
pub type USB2OTGLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB2ULPILPEN` reader - USB_PHY2 clocks enable during CSleep mode"]
pub type USB2ULPILPEN_R = crate::BitReader;
#[doc = "Field `USB2ULPILPEN` writer - USB_PHY2 clocks enable during CSleep mode"]
pub type USB2ULPILPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1maclpen(&self) -> ETH1MACLPEN_R {
        ETH1MACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1txlpen(&self) -> ETH1TXLPEN_R {
        ETH1TXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> ETH1RXLPEN_R {
        ETH1RXLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1otglpen(&self) -> USB1OTGLPEN_R {
        USB1OTGLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1ulpilpen(&self) -> USB1ULPILPEN_R {
        USB1ULPILPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2otglpen(&self) -> USB2OTGLPEN_R {
        USB2OTGLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB_PHY2 clocks enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2ulpilpen(&self) -> USB2ULPILPEN_R {
        USB2ULPILPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<C1_AHB1LPENR_SPEC, 0> {
        DMA1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<C1_AHB1LPENR_SPEC, 1> {
        DMA2LPEN_W::new(self)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<C1_AHB1LPENR_SPEC, 5> {
        ADC12LPEN_W::new(self)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth1maclpen(&mut self) -> ETH1MACLPEN_W<C1_AHB1LPENR_SPEC, 15> {
        ETH1MACLPEN_W::new(self)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth1txlpen(&mut self) -> ETH1TXLPEN_W<C1_AHB1LPENR_SPEC, 16> {
        ETH1TXLPEN_W::new(self)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn eth1rxlpen(&mut self) -> ETH1RXLPEN_W<C1_AHB1LPENR_SPEC, 17> {
        ETH1RXLPEN_W::new(self)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb1otglpen(&mut self) -> USB1OTGLPEN_W<C1_AHB1LPENR_SPEC, 25> {
        USB1OTGLPEN_W::new(self)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb1ulpilpen(&mut self) -> USB1ULPILPEN_W<C1_AHB1LPENR_SPEC, 26> {
        USB1ULPILPEN_W::new(self)
    }
    #[doc = "Bit 27 - USB2OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb2otglpen(&mut self) -> USB2OTGLPEN_W<C1_AHB1LPENR_SPEC, 27> {
        USB2OTGLPEN_W::new(self)
    }
    #[doc = "Bit 28 - USB_PHY2 clocks enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usb2ulpilpen(&mut self) -> USB2ULPILPEN_W<C1_AHB1LPENR_SPEC, 28> {
        USB2ULPILPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB1LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for C1_AHB1LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for C1_AHB1LPENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB1LPENR to value 0"]
impl crate::Resettable for C1_AHB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
