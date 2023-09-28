#[doc = "Register `MDIOS_DOUTR25` reader"]
pub type R = crate::R<MDIOS_DOUTR25_SPEC>;
#[doc = "Register `MDIOS_DOUTR25` writer"]
pub type W = crate::W<MDIOS_DOUTR25_SPEC>;
#[doc = "Field `DOUT25` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT25_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT25` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout25(&self) -> DOUT25_R {
        DOUT25_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout25(&mut self) -> DOUT25_W<MDIOS_DOUTR25_SPEC, 0> {
        DOUT25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIOS output data register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_doutr25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_doutr25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_DOUTR25_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr25::R`](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR25_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr25::W`](W) writer structure"]
impl crate::Writable for MDIOS_DOUTR25_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDIOS_DOUTR25 to value 0"]
impl crate::Resettable for MDIOS_DOUTR25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
