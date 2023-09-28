#[doc = "Register `PLL1FRACR` reader"]
pub type R = crate::R<PLL1FRACR_SPEC>;
#[doc = "Register `PLL1FRACR` writer"]
pub type W = crate::W<PLL1FRACR_SPEC>;
#[doc = "Field `FRACN1` reader - Fractional part of the multiplication factor for PLL1 VCO"]
pub type FRACN1_R = crate::FieldReader<u16>;
#[doc = "Field `FRACN1` writer - Fractional part of the multiplication factor for PLL1 VCO"]
pub type FRACN1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn fracn1(&self) -> FRACN1_R {
        FRACN1_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL1 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn fracn1(&mut self) -> FRACN1_W<PLL1FRACR_SPEC, 3> {
        FRACN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLL1 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1fracr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1fracr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1FRACR_SPEC;
impl crate::RegisterSpec for PLL1FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1fracr::R`](R) reader structure"]
impl crate::Readable for PLL1FRACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll1fracr::W`](W) writer structure"]
impl crate::Writable for PLL1FRACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1FRACR to value 0"]
impl crate::Resettable for PLL1FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
