#[doc = "Register `CPAR1` reader"]
pub type R = crate::R<CPAR1_SPEC>;
#[doc = "Register `CPAR1` writer"]
pub type W = crate::W<CPAR1_SPEC>;
#[doc = "Field `PA` reader - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
pub type PA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR1_SPEC, 0> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPAR1_SPEC;
impl crate::RegisterSpec for CPAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar1::R`](R) reader structure"]
impl crate::Readable for CPAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpar1::W`](W) writer structure"]
impl crate::Writable for CPAR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPAR1 to value 0"]
impl crate::Resettable for CPAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
