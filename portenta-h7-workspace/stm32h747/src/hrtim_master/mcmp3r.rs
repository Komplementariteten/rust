#[doc = "Register `MCMP3R` reader"]
pub type R = crate::R<MCMP3R_SPEC>;
#[doc = "Register `MCMP3R` writer"]
pub type W = crate::W<MCMP3R_SPEC>;
#[doc = "Field `MCMP3` reader - Master Timer Compare 3 value"]
pub type MCMP3_R = crate::FieldReader<u16>;
#[doc = "Field `MCMP3` writer - Master Timer Compare 3 value"]
pub type MCMP3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 3 value"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 3 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3(&mut self) -> MCMP3_W<MCMP3R_SPEC, 0> {
        MCMP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Timer Compare 3 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMP3R_SPEC;
impl crate::RegisterSpec for MCMP3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp3r::R`](R) reader structure"]
impl crate::Readable for MCMP3R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcmp3r::W`](W) writer structure"]
impl crate::Writable for MCMP3R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCMP3R to value 0"]
impl crate::Resettable for MCMP3R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
