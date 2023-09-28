#[doc = "Register `OPTKEYR` reader"]
pub type R = crate::R<OPTKEYR_SPEC>;
#[doc = "Register `OPTKEYR` writer"]
pub type W = crate::W<OPTKEYR_SPEC>;
#[doc = "Field `OPTKEYR` reader - Unlock key option bytes"]
pub type OPTKEYR_R = crate::FieldReader<u32>;
#[doc = "Field `OPTKEYR` writer - Unlock key option bytes"]
pub type OPTKEYR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&self) -> OPTKEYR_R {
        OPTKEYR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    #[must_use]
    pub fn optkeyr(&mut self) -> OPTKEYR_W<OPTKEYR_SPEC, 0> {
        OPTKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH option key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optkeyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optkeyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTKEYR_SPEC;
impl crate::RegisterSpec for OPTKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optkeyr::R`](R) reader structure"]
impl crate::Readable for OPTKEYR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure"]
impl crate::Writable for OPTKEYR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTKEYR to value 0"]
impl crate::Resettable for OPTKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
