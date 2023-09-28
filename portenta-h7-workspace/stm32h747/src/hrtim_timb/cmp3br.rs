#[doc = "Register `CMP3BR` reader"]
pub type R = crate::R<CMP3BR_SPEC>;
#[doc = "Register `CMP3BR` writer"]
pub type W = crate::W<CMP3BR_SPEC>;
#[doc = "Field `CMP3x` reader - Timerx Compare 3 value"]
pub type CMP3X_R = crate::FieldReader<u16>;
#[doc = "Field `CMP3x` writer - Timerx Compare 3 value"]
pub type CMP3X_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3x(&mut self) -> CMP3X_W<CMP3BR_SPEC, 0> {
        CMP3X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp3br::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp3br::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMP3BR_SPEC;
impl crate::RegisterSpec for CMP3BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp3br::R`](R) reader structure"]
impl crate::Readable for CMP3BR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmp3br::W`](W) writer structure"]
impl crate::Writable for CMP3BR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP3BR to value 0"]
impl crate::Resettable for CMP3BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
