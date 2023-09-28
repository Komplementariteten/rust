#[doc = "Register `DMASBMR` reader"]
pub type R = crate::R<DMASBMR_SPEC>;
#[doc = "Register `DMASBMR` writer"]
pub type W = crate::W<DMASBMR_SPEC>;
#[doc = "Field `FB` reader - Fixed Burst Length"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst Length"]
pub type FB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MB_R = crate::BitReader;
#[doc = "Field `RB` reader - Rebuild INCRx Burst"]
pub type RB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rebuild INCRx Burst"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fixed Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMASBMR_SPEC, 0> {
        FB_W::new(self)
    }
    #[doc = "Bit 12 - Address-Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<DMASBMR_SPEC, 12> {
        AAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasbmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasbmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMASBMR_SPEC;
impl crate::RegisterSpec for DMASBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasbmr::R`](R) reader structure"]
impl crate::Readable for DMASBMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmasbmr::W`](W) writer structure"]
impl crate::Writable for DMASBMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMASBMR to value 0x0101_0000"]
impl crate::Resettable for DMASBMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0000;
}
