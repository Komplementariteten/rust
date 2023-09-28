#[doc = "Register `D2CFGR` reader"]
pub type R = crate::R<D2CFGR_SPEC>;
#[doc = "Register `D2CFGR` writer"]
pub type W = crate::W<D2CFGR_SPEC>;
#[doc = "Field `D2PPRE1` reader - D2 domain APB1 prescaler"]
pub type D2PPRE1_R = crate::FieldReader;
#[doc = "Field `D2PPRE1` writer - D2 domain APB1 prescaler"]
pub type D2PPRE1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `D2PPRE2` reader - D2 domain APB2 prescaler"]
pub type D2PPRE2_R = crate::FieldReader;
#[doc = "Field `D2PPRE2` writer - D2 domain APB2 prescaler"]
pub type D2PPRE2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    pub fn d2ppre1(&self) -> D2PPRE1_R {
        D2PPRE1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    pub fn d2ppre2(&self) -> D2PPRE2_R {
        D2PPRE2_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d2ppre1(&mut self) -> D2PPRE1_W<D2CFGR_SPEC, 4> {
        D2PPRE1_W::new(self)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d2ppre2(&mut self) -> D2PPRE2_W<D2CFGR_SPEC, 8> {
        D2PPRE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 2 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d2cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d2cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2CFGR_SPEC;
impl crate::RegisterSpec for D2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2cfgr::R`](R) reader structure"]
impl crate::Readable for D2CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d2cfgr::W`](W) writer structure"]
impl crate::Writable for D2CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D2CFGR to value 0"]
impl crate::Resettable for D2CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
