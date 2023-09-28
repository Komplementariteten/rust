#[doc = "Register `MDIOS_DOUTR14` reader"]
pub type R = crate::R<MDIOS_DOUTR14_SPEC>;
#[doc = "Register `MDIOS_DOUTR14` writer"]
pub type W = crate::W<MDIOS_DOUTR14_SPEC>;
#[doc = "Field `DOUT14` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT14_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT14` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> DOUT14_W<MDIOS_DOUTR14_SPEC, 0> {
        DOUT14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS output data register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DOUTR14_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr14::R`](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr14::W`](W) writer structure"]
impl crate::Writable for MDIOS_DOUTR14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_DOUTR14 to value 0"]
impl crate::Resettable for MDIOS_DOUTR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
