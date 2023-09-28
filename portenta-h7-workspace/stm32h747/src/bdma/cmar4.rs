#[doc = "Register `CMAR4` reader"]
pub type R = crate::R<CMAR4_SPEC>;
#[doc = "Register `CMAR4` writer"]
pub type W = crate::W<CMAR4_SPEC>;
#[doc = "Field `MA` reader - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
pub type MA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR4_SPEC, 0> {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMAR4_SPEC;
impl crate::RegisterSpec for CMAR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar4::R`](R) reader structure"]
impl crate::Readable for CMAR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmar4::W`](W) writer structure"]
impl crate::Writable for CMAR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMAR4 to value 0"]
impl crate::Resettable for CMAR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
