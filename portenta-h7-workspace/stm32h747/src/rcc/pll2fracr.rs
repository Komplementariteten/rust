#[doc = "Register `PLL2FRACR` reader"]
pub type R = crate::R<PLL2FRACR_SPEC>;
#[doc = "Register `PLL2FRACR` writer"]
pub type W = crate::W<PLL2FRACR_SPEC>;
#[doc = "Field `FRACN2` reader - Fractional part of the multiplication factor for PLL VCO"]
pub type FRACN2_R = crate::FieldReader<u16>;
#[doc = "Field `FRACN2` writer - Fractional part of the multiplication factor for PLL VCO"]
pub type FRACN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL VCO"]
    #[inline(always)]
    pub fn fracn2(&self) -> FRACN2_R {
        FRACN2_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL VCO"]
    #[inline(always)]
    #[must_use]
    pub fn fracn2(&mut self) -> FRACN2_W<PLL2FRACR_SPEC, 3> {
        FRACN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLL2 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll2fracr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll2fracr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL2FRACR_SPEC;
impl crate::RegisterSpec for PLL2FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2fracr::R`](R) reader structure"]
impl crate::Readable for PLL2FRACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll2fracr::W`](W) writer structure"]
impl crate::Writable for PLL2FRACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL2FRACR to value 0"]
impl crate::Resettable for PLL2FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
