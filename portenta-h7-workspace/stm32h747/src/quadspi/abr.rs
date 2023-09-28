#[doc = "Register `ABR` reader"]
pub type R = crate::R<ABR_SPEC>;
#[doc = "Register `ABR` writer"]
pub type W = crate::W<ABR_SPEC>;
#[doc = "Field `ALTERNATE` reader - Alternate Bytes Optional data to be send to the external SPI device right after the address. This field can be written only when BUSY = 0."]
pub type ALTERNATE_R = crate::FieldReader<u32>;
#[doc = "Field `ALTERNATE` writer - Alternate Bytes Optional data to be send to the external SPI device right after the address. This field can be written only when BUSY = 0."]
pub type ALTERNATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Alternate Bytes Optional data to be send to the external SPI device right after the address. This field can be written only when BUSY = 0."]
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alternate Bytes Optional data to be send to the external SPI device right after the address. This field can be written only when BUSY = 0."]
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<ABR_SPEC, 0> {
        ALTERNATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "QUADSPI alternate bytes registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ABR_SPEC;
impl crate::RegisterSpec for ABR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`abr::R`](R) reader structure"]
impl crate::Readable for ABR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`abr::W`](W) writer structure"]
impl crate::Writable for ABR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABR to value 0"]
impl crate::Resettable for ABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
