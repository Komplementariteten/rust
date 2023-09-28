#[doc = "Register `PLL3FRACR` reader"]
pub type R = crate::R<PLL3FRACR_SPEC>;
#[doc = "Register `PLL3FRACR` writer"]
pub type W = crate::W<PLL3FRACR_SPEC>;
#[doc = "Field `FRACN3` reader - Fractional part of the multiplication factor for PLL3 VCO"]
pub type FRACN3_R = crate::FieldReader<u16>;
#[doc = "Field `FRACN3` writer - Fractional part of the multiplication factor for PLL3 VCO"]
pub type FRACN3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO"]
    #[inline(always)]
    pub fn fracn3(&self) -> FRACN3_R {
        FRACN3_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Fractional part of the multiplication factor for PLL3 VCO"]
    #[inline(always)]
    #[must_use]
    pub fn fracn3(&mut self) -> FRACN3_W<PLL3FRACR_SPEC, 3> {
        FRACN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC PLL3 Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll3fracr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll3fracr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL3FRACR_SPEC;
impl crate::RegisterSpec for PLL3FRACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll3fracr::R`](R) reader structure"]
impl crate::Readable for PLL3FRACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll3fracr::W`](W) writer structure"]
impl crate::Writable for PLL3FRACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL3FRACR to value 0"]
impl crate::Resettable for PLL3FRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
