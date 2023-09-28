#[doc = "Register `UR14` reader"]
pub type R = crate::R<UR14_SPEC>;
#[doc = "Register `UR14` writer"]
pub type W = crate::W<UR14_SPEC>;
#[doc = "Field `D1STPRST` reader - D1 Stop Reset"]
pub type D1STPRST_R = crate::BitReader;
#[doc = "Field `D1STPRST` writer - D1 Stop Reset"]
pub type D1STPRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D2SBRST` reader - D2 Standby Reset"]
pub type D2SBRST_R = crate::BitReader;
#[doc = "Field `D2SBRST` writer - D2 Standby Reset"]
pub type D2SBRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&self) -> D1STPRST_R {
        D1STPRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - D2 Standby Reset"]
    #[inline(always)]
    pub fn d2sbrst(&self) -> D2SBRST_R {
        D2SBRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d1stprst(&mut self) -> D1STPRST_W<UR14_SPEC, 0> {
        D1STPRST_W::new(self)
    }
    #[doc = "Bit 16 - D2 Standby Reset"]
    #[inline(always)]
    #[must_use]
    pub fn d2sbrst(&mut self) -> D2SBRST_W<UR14_SPEC, 16> {
        D2SBRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCFG user register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ur14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR14_SPEC;
impl crate::RegisterSpec for UR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur14::R`](R) reader structure"]
impl crate::Readable for UR14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ur14::W`](W) writer structure"]
impl crate::Writable for UR14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR14 to value 0"]
impl crate::Resettable for UR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
