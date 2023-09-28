#[doc = "Register `RXCRC` reader"]
pub type R = crate::R<RXCRC_SPEC>;
#[doc = "Register `RXCRC` writer"]
pub type W = crate::W<RXCRC_SPEC>;
#[doc = "Field `RXCRC` reader - CRC register for receiver"]
pub type RXCRC_R = crate::FieldReader<u32>;
#[doc = "Field `RXCRC` writer - CRC register for receiver"]
pub type RXCRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrc(&mut self) -> RXCRC_W<RXCRC_SPEC, 0> {
        RXCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Receiver CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRC_SPEC;
impl crate::RegisterSpec for RXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrc::R`](R) reader structure"]
impl crate::Readable for RXCRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxcrc::W`](W) writer structure"]
impl crate::Writable for RXCRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RXCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
