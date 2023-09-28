#[doc = "Register `TIM16_TISEL` reader"]
pub type R = crate::R<TIM16_TISEL_SPEC>;
#[doc = "Register `TIM16_TISEL` writer"]
pub type W = crate::W<TIM16_TISEL_SPEC>;
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
pub type TI1SEL_R = crate::FieldReader;
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
pub type TI1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TIM16_TISEL_SPEC, 0> {
        TI1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TIM16 input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim16_tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim16_tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM16_TISEL_SPEC;
impl crate::RegisterSpec for TIM16_TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim16_tisel::R`](R) reader structure"]
impl crate::Readable for TIM16_TISEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tim16_tisel::W`](W) writer structure"]
impl crate::Writable for TIM16_TISEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM16_TISEL to value 0"]
impl crate::Resettable for TIM16_TISEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
