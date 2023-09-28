#[doc = "Register `D1CFGR` reader"]
pub type R = crate::R<D1CFGR_SPEC>;
#[doc = "Register `D1CFGR` writer"]
pub type W = crate::W<D1CFGR_SPEC>;
#[doc = "Field `HPRE` reader - D1 domain AHB prescaler"]
pub type HPRE_R = crate::FieldReader;
#[doc = "Field `HPRE` writer - D1 domain AHB prescaler"]
pub type HPRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `D1PPRE` reader - D1 domain APB3 prescaler"]
pub type D1PPRE_R = crate::FieldReader;
#[doc = "Field `D1PPRE` writer - D1 domain APB3 prescaler"]
pub type D1PPRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `D1CPRE` reader - D1 domain Core prescaler"]
pub type D1CPRE_R = crate::FieldReader;
#[doc = "Field `D1CPRE` writer - D1 domain Core prescaler"]
pub type D1CPRE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    pub fn d1ppre(&self) -> D1PPRE_R {
        D1PPRE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    pub fn d1cpre(&self) -> D1CPRE_R {
        D1CPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<D1CFGR_SPEC, 0> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d1ppre(&mut self) -> D1PPRE_W<D1CFGR_SPEC, 4> {
        D1PPRE_W::new(self)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn d1cpre(&mut self) -> D1CPRE_W<D1CFGR_SPEC, 8> {
        D1CPRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Domain 1 Clock Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`d1cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`d1cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1CFGR_SPEC;
impl crate::RegisterSpec for D1CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1cfgr::R`](R) reader structure"]
impl crate::Readable for D1CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`d1cfgr::W`](W) writer structure"]
impl crate::Writable for D1CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D1CFGR to value 0"]
impl crate::Resettable for D1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
