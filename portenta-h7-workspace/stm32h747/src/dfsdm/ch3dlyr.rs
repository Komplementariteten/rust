#[doc = "Register `CH3DLYR` reader"]
pub type R = crate::R<CH3DLYR_SPEC>;
#[doc = "Register `CH3DLYR` writer"]
pub type W = crate::W<CH3DLYR_SPEC>;
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub type PLSSKP_R = crate::FieldReader;
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub type PLSSKP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<CH3DLYR_SPEC, 0> {
        PLSSKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3dlyr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3dlyr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3DLYR_SPEC;
impl crate::RegisterSpec for CH3DLYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3dlyr::R`](R) reader structure"]
impl crate::Readable for CH3DLYR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch3dlyr::W`](W) writer structure"]
impl crate::Writable for CH3DLYR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3DLYR to value 0"]
impl crate::Resettable for CH3DLYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
