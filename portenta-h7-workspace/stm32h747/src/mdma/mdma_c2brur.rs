#[doc = "Register `MDMA_C2BRUR` reader"]
pub type R = crate::R<MDMA_C2BRUR_SPEC>;
#[doc = "Register `MDMA_C2BRUR` writer"]
pub type W = crate::W<MDMA_C2BRUR_SPEC>;
#[doc = "Field `SUV` reader - source adresse update value"]
pub type SUV_R = crate::FieldReader<u16>;
#[doc = "Field `SUV` writer - source adresse update value"]
pub type SUV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DUV` reader - destination address update"]
pub type DUV_R = crate::FieldReader<u16>;
#[doc = "Field `DUV` writer - destination address update"]
pub type DUV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    pub fn suv(&self) -> SUV_R {
        SUV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    pub fn duv(&self) -> DUV_R {
        DUV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - source adresse update value"]
    #[inline(always)]
    #[must_use]
    pub fn suv(&mut self) -> SUV_W<MDMA_C2BRUR_SPEC, 0> {
        SUV_W::new(self)
    }
    #[doc = "Bits 16:31 - destination address update"]
    #[inline(always)]
    #[must_use]
    pub fn duv(&mut self) -> DUV_W<MDMA_C2BRUR_SPEC, 16> {
        DUV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDMA channel x Block Repeat address Update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c2brur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c2brur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C2BRUR_SPEC;
impl crate::RegisterSpec for MDMA_C2BRUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c2brur::R`](R) reader structure"]
impl crate::Readable for MDMA_C2BRUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdma_c2brur::W`](W) writer structure"]
impl crate::Writable for MDMA_C2BRUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C2BRUR to value 0"]
impl crate::Resettable for MDMA_C2BRUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
