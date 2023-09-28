#[doc = "Register `PCSEL` reader"]
pub type R = crate::R<PCSEL_SPEC>;
#[doc = "Register `PCSEL` writer"]
pub type W = crate::W<PCSEL_SPEC>;
#[doc = "Field `PCSEL` reader - Channel x (VINP\\[i\\]) pre selection"]
pub type PCSEL_R = crate::FieldReader<u32>;
#[doc = "Field `PCSEL` writer - Channel x (VINP\\[i\\]) pre selection"]
pub type PCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcsel(&mut self) -> PCSEL_W<PCSEL_SPEC, 0> {
        PCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC pre channel selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCSEL_SPEC;
impl crate::RegisterSpec for PCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcsel::R`](R) reader structure"]
impl crate::Readable for PCSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcsel::W`](W) writer structure"]
impl crate::Writable for PCSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCSEL to value 0"]
impl crate::Resettable for PCSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
