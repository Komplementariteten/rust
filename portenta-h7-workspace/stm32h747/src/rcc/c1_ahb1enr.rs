#[doc = "Register `C1_AHB1ENR` reader"]
pub type R = crate::R<C1_AHB1ENR_SPEC>;
#[doc = "Register `C1_AHB1ENR` writer"]
pub type W = crate::W<C1_AHB1ENR_SPEC>;
#[doc = "Field `DMA1EN` reader - DMA1 Clock Enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 Clock Enable"]
pub type DMA1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2EN` reader - DMA2 Clock Enable"]
pub type DMA2EN_R = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 Clock Enable"]
pub type DMA2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC12EN` reader - ADC1/2 Peripheral Clocks Enable"]
pub type ADC12EN_R = crate::BitReader;
#[doc = "Field `ADC12EN` writer - ADC1/2 Peripheral Clocks Enable"]
pub type ADC12EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1MACEN` reader - Ethernet MAC bus interface Clock Enable"]
pub type ETH1MACEN_R = crate::BitReader;
#[doc = "Field `ETH1MACEN` writer - Ethernet MAC bus interface Clock Enable"]
pub type ETH1MACEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1TXEN` reader - Ethernet Transmission Clock Enable"]
pub type ETH1TXEN_R = crate::BitReader;
#[doc = "Field `ETH1TXEN` writer - Ethernet Transmission Clock Enable"]
pub type ETH1TXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1RXEN` reader - Ethernet Reception Clock Enable"]
pub type ETH1RXEN_R = crate::BitReader;
#[doc = "Field `ETH1RXEN` writer - Ethernet Reception Clock Enable"]
pub type ETH1RXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB1OTGEN` reader - USB1OTG Peripheral Clocks Enable"]
pub type USB1OTGEN_R = crate::BitReader;
#[doc = "Field `USB1OTGEN` writer - USB1OTG Peripheral Clocks Enable"]
pub type USB1OTGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB1ULPIEN` reader - USB_PHY1 Clocks Enable"]
pub type USB1ULPIEN_R = crate::BitReader;
#[doc = "Field `USB1ULPIEN` writer - USB_PHY1 Clocks Enable"]
pub type USB1ULPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB2OTGEN` reader - USB2OTG Peripheral Clocks Enable"]
pub type USB2OTGEN_R = crate::BitReader;
#[doc = "Field `USB2OTGEN` writer - USB2OTG Peripheral Clocks Enable"]
pub type USB2OTGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB2ULPIEN` reader - USB_PHY2 Clocks Enable"]
pub type USB2ULPIEN_R = crate::BitReader;
#[doc = "Field `USB2ULPIEN` writer - USB_PHY2 Clocks Enable"]
pub type USB2ULPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otgen(&self) -> USB1OTGEN_R {
        USB1OTGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1ulpien(&self) -> USB1ULPIEN_R {
        USB1ULPIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otgen(&self) -> USB2OTGEN_R {
        USB2OTGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB_PHY2 Clocks Enable"]
    #[inline(always)]
    pub fn usb2ulpien(&self) -> USB2ULPIEN_R {
        USB2ULPIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<C1_AHB1ENR_SPEC, 0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<C1_AHB1ENR_SPEC, 1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<C1_AHB1ENR_SPEC, 5> {
        ADC12EN_W::new(self)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W<C1_AHB1ENR_SPEC, 15> {
        ETH1MACEN_W::new(self)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W<C1_AHB1ENR_SPEC, 16> {
        ETH1TXEN_W::new(self)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W<C1_AHB1ENR_SPEC, 17> {
        ETH1RXEN_W::new(self)
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb1otgen(&mut self) -> USB1OTGEN_W<C1_AHB1ENR_SPEC, 25> {
        USB1OTGEN_W::new(self)
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb1ulpien(&mut self) -> USB1ULPIEN_W<C1_AHB1ENR_SPEC, 26> {
        USB1ULPIEN_W::new(self)
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb2otgen(&mut self) -> USB2OTGEN_W<C1_AHB1ENR_SPEC, 27> {
        USB2OTGEN_W::new(self)
    }
    #[doc = "Bit 28 - USB_PHY2 Clocks Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usb2ulpien(&mut self) -> USB2ULPIEN_W<C1_AHB1ENR_SPEC, 28> {
        USB2ULPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB1 Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_ahb1enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_ahb1enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_AHB1ENR_SPEC;
impl crate::RegisterSpec for C1_AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb1enr::R`](R) reader structure"]
impl crate::Readable for C1_AHB1ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb1enr::W`](W) writer structure"]
impl crate::Writable for C1_AHB1ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_AHB1ENR to value 0"]
impl crate::Resettable for C1_AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
