#[doc = "Register `MACSPI0R` reader"]
pub type R = crate::R<MACSPI0R_SPEC>;
#[doc = "Register `MACSPI0R` writer"]
pub type W = crate::W<MACSPI0R_SPEC>;
#[doc = "Field `SPI0` reader - SPI0"]
pub type SPI0_R = crate::FieldReader<u32>;
#[doc = "Field `SPI0` writer - SPI0"]
pub type SPI0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI0"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<MACSPI0R_SPEC, 0> {
        SPI0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PTP Source Port Identity 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSPI0R_SPEC;
impl crate::RegisterSpec for MACSPI0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi0r::R`](R) reader structure"]
impl crate::Readable for MACSPI0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macspi0r::W`](W) writer structure"]
impl crate::Writable for MACSPI0R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSPI0R to value 0"]
impl crate::Resettable for MACSPI0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
