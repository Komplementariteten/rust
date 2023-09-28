#[doc = "Register `KEYR1` reader"]
pub type R = crate::R<KEYR1_SPEC>;
#[doc = "Register `KEYR1` writer"]
pub type W = crate::W<KEYR1_SPEC>;
#[doc = "Field `KEYR1` reader - Bank 1 access configuration unlock key"]
pub type KEYR1_R = crate::FieldReader<u32>;
#[doc = "Field `KEYR1` writer - Bank 1 access configuration unlock key"]
pub type KEYR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr1(&self) -> KEYR1_R {
        KEYR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    #[must_use]
    pub fn keyr1(&mut self) -> KEYR1_W<KEYR1_SPEC, 0> {
        KEYR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH key register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR1_SPEC;
impl crate::RegisterSpec for KEYR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr1::R`](R) reader structure"]
impl crate::Readable for KEYR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`keyr1::W`](W) writer structure"]
impl crate::Writable for KEYR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYR1 to value 0"]
impl crate::Resettable for KEYR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
