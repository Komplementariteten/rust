#[doc = "Register `LHTR1` reader"]
pub type R = crate::R<LHTR1_SPEC>;
#[doc = "Register `LHTR1` writer"]
pub type W = crate::W<LHTR1_SPEC>;
#[doc = "Field `LHTR1` reader - ADC analog watchdog 2 threshold low"]
pub type LHTR1_R = crate::FieldReader<u32>;
#[doc = "Field `LHTR1` writer - ADC analog watchdog 2 threshold low"]
pub type LHTR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lhtr1(&self) -> LHTR1_R {
        LHTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn lhtr1(&mut self) -> LHTR1_W<LHTR1_SPEC, 0> {
        LHTR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lhtr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lhtr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LHTR1_SPEC;
impl crate::RegisterSpec for LHTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lhtr1::R`](R) reader structure"]
impl crate::Readable for LHTR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lhtr1::W`](W) writer structure"]
impl crate::Writable for LHTR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LHTR1 to value 0x0fff_0000"]
impl crate::Resettable for LHTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
