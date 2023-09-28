#[doc = "Register `OFR1` reader"]
pub type R = crate::R<OFR1_SPEC>;
#[doc = "Register `OFR1` writer"]
pub type W = crate::W<OFR1_SPEC>;
#[doc = "Field `OFFSET1` reader - ADC offset number 1 offset level"]
pub type OFFSET1_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET1` writer - ADC offset number 1 offset level"]
pub type OFFSET1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
#[doc = "Field `OFFSET1_CH` reader - ADC offset number 1 channel selection"]
pub type OFFSET1_CH_R = crate::FieldReader;
#[doc = "Field `OFFSET1_CH` writer - ADC offset number 1 channel selection"]
pub type OFFSET1_CH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SSATE` reader - ADC offset number 1 enable"]
pub type SSATE_R = crate::BitReader;
#[doc = "Field `SSATE` writer - ADC offset number 1 enable"]
pub type SSATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - ADC offset number 1 enable"]
    #[inline(always)]
    pub fn ssate(&self) -> SSATE_R {
        SSATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC offset number 1 offset level"]
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<OFR1_SPEC, 0> {
        OFFSET1_W::new(self)
    }
    #[doc = "Bits 26:30 - ADC offset number 1 channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<OFR1_SPEC, 26> {
        OFFSET1_CH_W::new(self)
    }
    #[doc = "Bit 31 - ADC offset number 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssate(&mut self) -> SSATE_W<OFR1_SPEC, 31> {
        SSATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC offset number 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OFR1_SPEC;
impl crate::RegisterSpec for OFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ofr1::R`](R) reader structure"]
impl crate::Readable for OFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ofr1::W`](W) writer structure"]
impl crate::Writable for OFR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFR1 to value 0"]
impl crate::Resettable for OFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
