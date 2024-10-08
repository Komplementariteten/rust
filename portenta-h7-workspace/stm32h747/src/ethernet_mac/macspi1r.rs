#[doc = "Register `MACSPI1R` reader"]
pub type R = crate::R<MACSPI1R_SPEC>;
#[doc = "Register `MACSPI1R` writer"]
pub type W = crate::W<MACSPI1R_SPEC>;
#[doc = "Field `SPI1` reader - SPI1"]
pub type SPI1_R = crate::FieldReader<u32>;
#[doc = "Field `SPI1` writer - SPI1"]
pub type SPI1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<MACSPI1R_SPEC, 0> {
        SPI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTP Source port identity 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSPI1R_SPEC;
impl crate::RegisterSpec for MACSPI1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi1r::R`](R) reader structure"]
impl crate::Readable for MACSPI1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macspi1r::W`](W) writer structure"]
impl crate::Writable for MACSPI1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSPI1R to value 0"]
impl crate::Resettable for MACSPI1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
