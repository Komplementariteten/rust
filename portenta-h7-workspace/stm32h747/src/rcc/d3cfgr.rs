#[doc = "Register `D3CFGR` reader"]
pub type R = crate::R<D3CFGR_SPEC>;
#[doc = "Register `D3CFGR` writer"]
pub type W = crate::W<D3CFGR_SPEC>;
#[doc = "Field `D3PPRE` reader - D3 domain APB4 prescaler"]
pub type D3PPRE_R = crate::FieldReader;
#[doc = "Field `D3PPRE` writer - D3 domain APB4 prescaler"]
pub type D3PPRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&self) -> D3PPRE_R {
        D3PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d3ppre(&mut self) -> D3PPRE_W<D3CFGR_SPEC, 4> {
        D3PPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 3 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d3cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d3cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3CFGR_SPEC;
impl crate::RegisterSpec for D3CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3cfgr::R`](R) reader structure"]
impl crate::Readable for D3CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d3cfgr::W`](W) writer structure"]
impl crate::Writable for D3CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D3CFGR to value 0"]
impl crate::Resettable for D3CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
