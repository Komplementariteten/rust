#[doc = "Register `MCMP2R` reader"]
pub type R = crate::R<MCMP2R_SPEC>;
#[doc = "Register `MCMP2R` writer"]
pub type W = crate::W<MCMP2R_SPEC>;
#[doc = "Field `MCMP2` reader - Master Timer Compare 2 value"]
pub type MCMP2_R = crate::FieldReader<u16>;
#[doc = "Field `MCMP2` writer - Master Timer Compare 2 value"]
pub type MCMP2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Timer Compare 2 value"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> MCMP2_W<MCMP2R_SPEC, 0> {
        MCMP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Master Timer Compare 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcmp2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcmp2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCMP2R_SPEC;
impl crate::RegisterSpec for MCMP2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmp2r::R`](R) reader structure"]
impl crate::Readable for MCMP2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcmp2r::W`](W) writer structure"]
impl crate::Writable for MCMP2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCMP2R to value 0"]
impl crate::Resettable for MCMP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
