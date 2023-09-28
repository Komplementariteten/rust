#[doc = "Register `C1_RSR` reader"]
pub type R = crate::R<C1_RSR_SPEC>;
#[doc = "Register `C1_RSR` writer"]
pub type W = crate::W<C1_RSR_SPEC>;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPURSTF` reader - CPU reset flag"]
pub type CPURSTF_R = crate::BitReader;
#[doc = "Field `CPURSTF` writer - CPU reset flag"]
pub type CPURSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D1RSTF` reader - D1 domain power switch reset flag"]
pub type D1RSTF_R = crate::BitReader;
#[doc = "Field `D1RSTF` writer - D1 domain power switch reset flag"]
pub type D1RSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `D2RSTF` reader - D2 domain power switch reset flag"]
pub type D2RSTF_R = crate::BitReader;
#[doc = "Field `D2RSTF` writer - D2 domain power switch reset flag"]
pub type D2RSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BORRSTF_R = crate::BitReader;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub type BORRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PINRSTF` reader - Pin reset flag (NRST)"]
pub type PINRSTF_R = crate::BitReader;
#[doc = "Field `PINRSTF` writer - Pin reset flag (NRST)"]
pub type PINRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub type PORRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SFTRSTF` reader - System reset from CPU reset flag"]
pub type SFTRSTF_R = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - System reset from CPU reset flag"]
pub type SFTRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IWDG1RSTF` reader - Independent Watchdog reset flag"]
pub type IWDG1RSTF_R = crate::BitReader;
#[doc = "Field `IWDG1RSTF` writer - Independent Watchdog reset flag"]
pub type IWDG1RSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDG1RSTF` reader - Window Watchdog reset flag"]
pub type WWDG1RSTF_R = crate::BitReader;
#[doc = "Field `WWDG1RSTF` writer - Window Watchdog reset flag"]
pub type WWDG1RSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPWRRSTF` reader - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LPWRRSTF_R = crate::BitReader;
#[doc = "Field `LPWRRSTF` writer - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LPWRRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&self) -> CPURSTF_R {
        CPURSTF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&self) -> D1RSTF_R {
        D1RSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&self) -> D2RSTF_R {
        D2RSTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<C1_RSR_SPEC, 16> {
        RMVF_W::new(self)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn cpurstf(&mut self) -> CPURSTF_W<C1_RSR_SPEC, 17> {
        CPURSTF_W::new(self)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn d1rstf(&mut self) -> D1RSTF_W<C1_RSR_SPEC, 19> {
        D1RSTF_W::new(self)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn d2rstf(&mut self) -> D2RSTF_W<C1_RSR_SPEC, 20> {
        D2RSTF_W::new(self)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<C1_RSR_SPEC, 21> {
        BORRSTF_W::new(self)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<C1_RSR_SPEC, 22> {
        PINRSTF_W::new(self)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<C1_RSR_SPEC, 23> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<C1_RSR_SPEC, 24> {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<C1_RSR_SPEC, 26> {
        IWDG1RSTF_W::new(self)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W<C1_RSR_SPEC, 28> {
        WWDG1RSTF_W::new(self)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<C1_RSR_SPEC, 30> {
        LPWRRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Reset Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1_rsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1_rsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1_RSR_SPEC;
impl crate::RegisterSpec for C1_RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_rsr::R`](R) reader structure"]
impl crate::Readable for C1_RSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c1_rsr::W`](W) writer structure"]
impl crate::Writable for C1_RSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1_RSR to value 0"]
impl crate::Resettable for C1_RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
