#[doc = "Register `AWD3CR` reader"]
pub type R = crate::R<AWD3CR_SPEC>;
#[doc = "Register `AWD3CR` writer"]
pub type W = crate::W<AWD3CR_SPEC>;
#[doc = "Field `AWD3CH` reader - ADC analog watchdog 3 monitored channel selection"]
pub type AWD3CH_R = crate::FieldReader<u32>;
#[doc = "Field `AWD3CH` writer - ADC analog watchdog 3 monitored channel selection"]
pub type AWD3CH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 1:20 - ADC analog watchdog 3 monitored channel selection"]
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new((self.bits >> 1) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 1:20 - ADC analog watchdog 3 monitored channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch(&mut self) -> AWD3CH_W<AWD3CR_SPEC, 1> {
        AWD3CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC analog watchdog 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWD3CR_SPEC;
impl crate::RegisterSpec for AWD3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awd3cr::R`](R) reader structure"]
impl crate::Readable for AWD3CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure"]
impl crate::Writable for AWD3CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for AWD3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
