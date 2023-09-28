#[doc = "Register `MACSPI2R` reader"]
pub type R = crate::R<MACSPI2R_SPEC>;
#[doc = "Register `MACSPI2R` writer"]
pub type W = crate::W<MACSPI2R_SPEC>;
#[doc = "Field `SPI2` reader - SPI2"]
pub type SPI2_R = crate::FieldReader<u16>;
#[doc = "Field `SPI2` writer - SPI2"]
pub type SPI2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<MACSPI2R_SPEC, 0> {
        SPI2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTP Source port identity 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSPI2R_SPEC;
impl crate::RegisterSpec for MACSPI2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi2r::R`](R) reader structure"]
impl crate::Readable for MACSPI2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macspi2r::W`](W) writer structure"]
impl crate::Writable for MACSPI2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSPI2R to value 0"]
impl crate::Resettable for MACSPI2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
