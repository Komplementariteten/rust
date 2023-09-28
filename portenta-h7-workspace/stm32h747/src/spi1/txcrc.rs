#[doc = "Register `TXCRC` reader"]
pub type R = crate::R<TXCRC_SPEC>;
#[doc = "Register `TXCRC` writer"]
pub type W = crate::W<TXCRC_SPEC>;
#[doc = "Field `TXCRC` reader - CRC register for transmitter"]
pub type TXCRC_R = crate::FieldReader<u32>;
#[doc = "Field `TXCRC` writer - CRC register for transmitter"]
pub type TXCRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn txcrc(&mut self) -> TXCRC_W<TXCRC_SPEC, 0> {
        TXCRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmitter CRC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCRC_SPEC;
impl crate::RegisterSpec for TXCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrc::R`](R) reader structure"]
impl crate::Readable for TXCRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txcrc::W`](W) writer structure"]
impl crate::Writable for TXCRC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TXCRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
