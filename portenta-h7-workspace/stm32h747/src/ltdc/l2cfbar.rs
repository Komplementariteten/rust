#[doc = "Register `L2CFBAR` reader"]
pub type R = crate::R<L2CFBAR_SPEC>;
#[doc = "Register `L2CFBAR` writer"]
pub type W = crate::W<L2CFBAR_SPEC>;
#[doc = "Field `CFBADD` reader - Color Frame Buffer Start Address"]
pub type CFBADD_R = crate::FieldReader<u32>;
#[doc = "Field `CFBADD` writer - Color Frame Buffer Start Address"]
pub type CFBADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn cfbadd(&mut self) -> CFBADD_W<L2CFBAR_SPEC, 0> {
        CFBADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layerx Color Frame Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2cfbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2cfbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CFBAR_SPEC;
impl crate::RegisterSpec for L2CFBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2cfbar::R`](R) reader structure"]
impl crate::Readable for L2CFBAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2cfbar::W`](W) writer structure"]
impl crate::Writable for L2CFBAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2CFBAR to value 0"]
impl crate::Resettable for L2CFBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
