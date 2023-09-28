#[doc = "Register `CREL` reader"]
pub type R = crate::R<CREL_SPEC>;
#[doc = "Register `CREL` writer"]
pub type W = crate::W<CREL_SPEC>;
#[doc = "Field `DAY` reader - Time Stamp Day"]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `DAY` writer - Time Stamp Day"]
pub type DAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `MON` reader - Time Stamp Month"]
pub type MON_R = crate::FieldReader;
#[doc = "Field `MON` writer - Time Stamp Month"]
pub type MON_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `YEAR` reader - Time Stamp Year"]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `YEAR` writer - Time Stamp Year"]
pub type YEAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SUBSTEP_R = crate::FieldReader;
#[doc = "Field `SUBSTEP` writer - Sub-step of Core Release"]
pub type SUBSTEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `STEP` writer - Step of Core Release"]
pub type STEP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `REL` reader - Core Release"]
pub type REL_R = crate::FieldReader;
#[doc = "Field `REL` writer - Core Release"]
pub type REL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<CREL_SPEC, 0> {
        DAY_W::new(self)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<CREL_SPEC, 8> {
        MON_W::new(self)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<CREL_SPEC, 16> {
        YEAR_W::new(self)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn substep(&mut self) -> SUBSTEP_W<CREL_SPEC, 20> {
        SUBSTEP_W::new(self)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<CREL_SPEC, 24> {
        STEP_W::new(self)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    #[must_use]
    pub fn rel(&mut self) -> REL_W<CREL_SPEC, 28> {
        REL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Calibration Unit Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CREL_SPEC;
impl crate::RegisterSpec for CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CREL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crel::W`](W) writer structure"]
impl crate::Writable for CREL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
