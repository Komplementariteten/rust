#[doc = "Register `MDIOS_DOUTR21` reader"]
pub type R = crate::R<MDIOS_DOUTR21_SPEC>;
#[doc = "Register `MDIOS_DOUTR21` writer"]
pub type W = crate::W<MDIOS_DOUTR21_SPEC>;
#[doc = "Field `DOUT21` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT21_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT21` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT21_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout21(&self) -> DOUT21_R {
        DOUT21_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout21(&mut self) -> DOUT21_W<MDIOS_DOUTR21_SPEC, 0> {
        DOUT21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS output data register 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DOUTR21_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr21::R`](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR21_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr21::W`](W) writer structure"]
impl crate::Writable for MDIOS_DOUTR21_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_DOUTR21 to value 0"]
impl crate::Resettable for MDIOS_DOUTR21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
