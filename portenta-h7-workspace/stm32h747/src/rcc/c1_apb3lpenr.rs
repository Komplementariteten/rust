#[doc = "Register `C1_APB3LPENR` reader"]
pub type R = crate::R<C1_APB3LPENR_SPEC>;
#[doc = "Register `C1_APB3LPENR` writer"]
pub type W = crate::W<C1_APB3LPENR_SPEC>;
#[doc = "Field `LTDCLPEN` reader - LTDC peripheral clock enable during CSleep mode"]
pub type LTDCLPEN_R = crate::BitReader;
#[doc = "Field `LTDCLPEN` writer - LTDC peripheral clock enable during CSleep mode"]
pub type LTDCLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDG1LPEN` reader - WWDG1 Clock Enable During CSleep Mode"]
pub type WWDG1LPEN_R = crate::BitReader;
#[doc = "Field `WWDG1LPEN` writer - WWDG1 Clock Enable During CSleep Mode"]
pub type WWDG1LPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn wwdg1lpen(&self) -> WWDG1LPEN_R {
        WWDG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<C1_APB3LPENR_SPEC, 3> {
        LTDCLPEN_W::new(self)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1lpen(&mut self) -> WWDG1LPEN_W<C1_APB3LPENR_SPEC, 6> {
        WWDG1LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_apb3lpenr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_apb3lpenr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_APB3LPENR_SPEC;
impl crate::RegisterSpec for C1_APB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb3lpenr::R`](R) reader structure"]
impl crate::Readable for C1_APB3LPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_apb3lpenr::W`](W) writer structure"]
impl crate::Writable for C1_APB3LPENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_APB3LPENR to value 0"]
impl crate::Resettable for C1_APB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
