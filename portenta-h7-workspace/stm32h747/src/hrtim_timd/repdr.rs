#[doc = "Register `REPDR` reader"]
pub type R = crate::R<REPDR_SPEC>;
#[doc = "Register `REPDR` writer"]
pub type W = crate::W<REPDR_SPEC>;
#[doc = "Field `REPx` reader - Timerx Repetition counter value"]
pub type REPX_R = crate::FieldReader;
#[doc = "Field `REPx` writer - Timerx Repetition counter value"]
pub type REPX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    #[must_use]
    pub fn repx(&mut self) -> REPX_W<REPDR_SPEC, 0> {
        REPX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`repdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`repdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REPDR_SPEC;
impl crate::RegisterSpec for REPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`repdr::R`](R) reader structure"]
impl crate::Readable for REPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`repdr::W`](W) writer structure"]
impl crate::Writable for REPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REPDR to value 0"]
impl crate::Resettable for REPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
