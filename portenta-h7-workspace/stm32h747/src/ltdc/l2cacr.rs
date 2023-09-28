#[doc = "Register `L2CACR` reader"]
pub type R = crate::R<L2CACR_SPEC>;
#[doc = "Register `L2CACR` writer"]
pub type W = crate::W<L2CACR_SPEC>;
#[doc = "Field `CONSTA` reader - Constant Alpha"]
pub type CONSTA_R = crate::FieldReader;
#[doc = "Field `CONSTA` writer - Constant Alpha"]
pub type CONSTA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    #[must_use]
    pub fn consta(&mut self) -> CONSTA_W<L2CACR_SPEC, 0> {
        CONSTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CACR_SPEC;
impl crate::RegisterSpec for L2CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2cacr::R`](R) reader structure"]
impl crate::Readable for L2CACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2cacr::W`](W) writer structure"]
impl crate::Writable for L2CACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2CACR to value 0"]
impl crate::Resettable for L2CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
