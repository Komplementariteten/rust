#[doc = "Register `MPER` reader"]
pub type R = crate::R<MPER_SPEC>;
#[doc = "Register `MPER` writer"]
pub type W = crate::W<MPER_SPEC>;
#[doc = "Field `MPER` reader - Master Timer Period value"]
pub type MPER_R = crate::FieldReader<u16>;
#[doc = "Field `MPER` writer - Master Timer Period value"]
pub type MPER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Period value"]
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<MPER_SPEC, 0> {
        MPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Timer Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPER_SPEC;
impl crate::RegisterSpec for MPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mper::R`](R) reader structure"]
impl crate::Readable for MPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mper::W`](W) writer structure"]
impl crate::Writable for MPER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPER to value 0xffff"]
impl crate::Resettable for MPER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
