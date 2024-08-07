#[doc = "Register `PWRCR` reader"]
pub type R = crate::R<PWRCR_SPEC>;
#[doc = "Register `PWRCR` writer"]
pub type W = crate::W<PWRCR_SPEC>;
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub type ODEN_R = crate::BitReader;
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub type ODEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    #[must_use]
    pub fn oden(&mut self) -> ODEN_W<PWRCR_SPEC, 0> {
        ODEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SYSCFG power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCR_SPEC;
impl crate::RegisterSpec for PWRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcr::R`](R) reader structure"]
impl crate::Readable for PWRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure"]
impl crate::Writable for PWRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
