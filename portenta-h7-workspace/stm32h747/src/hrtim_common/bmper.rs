#[doc = "Register `BMPER` reader"]
pub type R = crate::R<BMPER_SPEC>;
#[doc = "Register `BMPER` writer"]
pub type W = crate::W<BMPER_SPEC>;
#[doc = "Field `BMPER` reader - Burst mode Period"]
pub type BMPER_R = crate::FieldReader<u16>;
#[doc = "Field `BMPER` writer - Burst mode Period"]
pub type BMPER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    pub fn bmper(&self) -> BMPER_R {
        BMPER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Burst mode Period"]
    #[inline(always)]
    #[must_use]
    pub fn bmper(&mut self) -> BMPER_W<BMPER_SPEC, 0> {
        BMPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Burst Mode Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMPER_SPEC;
impl crate::RegisterSpec for BMPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmper::R`](R) reader structure"]
impl crate::Readable for BMPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmper::W`](W) writer structure"]
impl crate::Writable for BMPER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BMPER to value 0"]
impl crate::Resettable for BMPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
