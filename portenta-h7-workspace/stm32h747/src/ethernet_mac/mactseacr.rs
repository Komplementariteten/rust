#[doc = "Register `MACTSEACR` reader"]
pub type R = crate::R<MACTSEACR_SPEC>;
#[doc = "Register `MACTSEACR` writer"]
pub type W = crate::W<MACTSEACR_SPEC>;
#[doc = "Field `OSTEAC` reader - OSTEAC"]
pub type OSTEAC_R = crate::FieldReader<u32>;
#[doc = "Field `OSTEAC` writer - OSTEAC"]
pub type OSTEAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    #[must_use]
    pub fn osteac(&mut self) -> OSTEAC_W<MACTSEACR_SPEC, 0> {
        OSTEAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp Egress asymmetric correction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactseacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactseacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSEACR_SPEC;
impl crate::RegisterSpec for MACTSEACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactseacr::R`](R) reader structure"]
impl crate::Readable for MACTSEACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mactseacr::W`](W) writer structure"]
impl crate::Writable for MACTSEACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSEACR to value 0"]
impl crate::Resettable for MACTSEACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
