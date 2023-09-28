#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<AHB1RSTR_SPEC>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<AHB1RSTR_SPEC>;
#[doc = "Field `DMA1RST` reader - DMA1 block reset"]
pub type DMA1RST_R = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 block reset"]
pub type DMA1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMA2RST` reader - DMA2 block reset"]
pub type DMA2RST_R = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 block reset"]
pub type DMA2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC12RST` reader - ADC1&amp;2 block reset"]
pub type ADC12RST_R = crate::BitReader;
#[doc = "Field `ADC12RST` writer - ADC1&amp;2 block reset"]
pub type ADC12RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETH1MACRST` reader - ETH1MAC block reset"]
pub type ETH1MACRST_R = crate::BitReader;
#[doc = "Field `ETH1MACRST` writer - ETH1MAC block reset"]
pub type ETH1MACRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB1OTGRST` reader - USB1OTG block reset"]
pub type USB1OTGRST_R = crate::BitReader;
#[doc = "Field `USB1OTGRST` writer - USB1OTG block reset"]
pub type USB1OTGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB2OTGRST` reader - USB2OTG block reset"]
pub type USB2OTGRST_R = crate::BitReader;
#[doc = "Field `USB2OTGRST` writer - USB2OTG block reset"]
pub type USB2OTGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1&amp;2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&self) -> ETH1MACRST_R {
        ETH1MACRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&self) -> USB1OTGRST_R {
        USB1OTGRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&self) -> USB2OTGRST_R {
        USB2OTGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<AHB1RSTR_SPEC, 0> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<AHB1RSTR_SPEC, 1> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 5 - ADC1&amp;2 block reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc12rst(&mut self) -> ADC12RST_W<AHB1RSTR_SPEC, 5> {
        ADC12RST_W::new(self)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn eth1macrst(&mut self) -> ETH1MACRST_W<AHB1RSTR_SPEC, 15> {
        ETH1MACRST_W::new(self)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usb1otgrst(&mut self) -> USB1OTGRST_W<AHB1RSTR_SPEC, 25> {
        USB1OTGRST_W::new(self)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    #[must_use]
    pub fn usb2otgrst(&mut self) -> USB2OTGRST_W<AHB1RSTR_SPEC, 27> {
        USB2OTGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb1rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
