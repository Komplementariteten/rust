#[doc = "Register `BMCMPR6` reader"]
pub type R = crate::R<BMCMPR6_SPEC>;
#[doc = "Register `BMCMPR6` writer"]
pub type W = crate::W<BMCMPR6_SPEC>;
#[doc = "Field `BMCMP` reader - BMCMP"]
pub type BMCMP_R = crate::FieldReader<u16>;
#[doc = "Field `BMCMP` writer - BMCMP"]
pub type BMCMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BMCMP_R {
        BMCMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    #[must_use]
    pub fn bmcmp(&mut self) -> BMCMP_W<BMCMPR6_SPEC, 0> {
        BMCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "BMCMPR6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmcmpr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmcmpr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMCMPR6_SPEC;
impl crate::RegisterSpec for BMCMPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcmpr6::R`](R) reader structure"]
impl crate::Readable for BMCMPR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmcmpr6::W`](W) writer structure"]
impl crate::Writable for BMCMPR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMCMPR6 to value 0"]
impl crate::Resettable for BMCMPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
