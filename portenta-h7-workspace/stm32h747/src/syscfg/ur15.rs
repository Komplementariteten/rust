#[doc = "Register `UR15` reader"]
pub type R = crate::R<UR15_SPEC>;
#[doc = "Register `UR15` writer"]
pub type W = crate::W<UR15_SPEC>;
#[doc = "Field `D2STPRST` reader - D2 Stop Reset"]
pub type D2STPRST_R = crate::BitReader;
#[doc = "Field `D2STPRST` writer - D2 Stop Reset"]
pub type D2STPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode"]
pub type FZIWDGSTB_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    pub fn d2stprst(&self) -> D2STPRST_R {
        D2STPRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FZIWDGSTB_R {
        FZIWDGSTB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d2stprst(&mut self) -> D2STPRST_W<UR15_SPEC, 0> {
        D2STPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCFG user register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR15_SPEC;
impl crate::RegisterSpec for UR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur15::R`](R) reader structure"]
impl crate::Readable for UR15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ur15::W`](W) writer structure"]
impl crate::Writable for UR15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR15 to value 0"]
impl crate::Resettable for UR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
