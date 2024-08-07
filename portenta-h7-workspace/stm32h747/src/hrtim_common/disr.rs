#[doc = "Register `DISR` reader"]
pub type R = crate::R<DISR_SPEC>;
#[doc = "Register `DISR` writer"]
pub type W = crate::W<DISR_SPEC>;
#[doc = "Field `TA1ODIS` reader - TA1ODIS"]
pub type TA1ODIS_R = crate::BitReader;
#[doc = "Field `TA1ODIS` writer - TA1ODIS"]
pub type TA1ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TA2ODIS` reader - TA2ODIS"]
pub type TA2ODIS_R = crate::BitReader;
#[doc = "Field `TA2ODIS` writer - TA2ODIS"]
pub type TA2ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TB1ODIS` reader - TB1ODIS"]
pub type TB1ODIS_R = crate::BitReader;
#[doc = "Field `TB1ODIS` writer - TB1ODIS"]
pub type TB1ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TB2ODIS` reader - TB2ODIS"]
pub type TB2ODIS_R = crate::BitReader;
#[doc = "Field `TB2ODIS` writer - TB2ODIS"]
pub type TB2ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC1ODIS` reader - TC1ODIS"]
pub type TC1ODIS_R = crate::BitReader;
#[doc = "Field `TC1ODIS` writer - TC1ODIS"]
pub type TC1ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TC2ODIS` reader - TC2ODIS"]
pub type TC2ODIS_R = crate::BitReader;
#[doc = "Field `TC2ODIS` writer - TC2ODIS"]
pub type TC2ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TD1ODIS` reader - TD1ODIS"]
pub type TD1ODIS_R = crate::BitReader;
#[doc = "Field `TD1ODIS` writer - TD1ODIS"]
pub type TD1ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TD2ODIS` reader - TD2ODIS"]
pub type TD2ODIS_R = crate::BitReader;
#[doc = "Field `TD2ODIS` writer - TD2ODIS"]
pub type TD2ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TE1ODIS` reader - TE1ODIS"]
pub type TE1ODIS_R = crate::BitReader;
#[doc = "Field `TE1ODIS` writer - TE1ODIS"]
pub type TE1ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TE2ODIS` reader - TE2ODIS"]
pub type TE2ODIS_R = crate::BitReader;
#[doc = "Field `TE2ODIS` writer - TE2ODIS"]
pub type TE2ODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&self) -> TA1ODIS_R {
        TA1ODIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&self) -> TA2ODIS_R {
        TA2ODIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&self) -> TB1ODIS_R {
        TB1ODIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&self) -> TB2ODIS_R {
        TB2ODIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&self) -> TC1ODIS_R {
        TC1ODIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&self) -> TC2ODIS_R {
        TC2ODIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&self) -> TD1ODIS_R {
        TD1ODIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&self) -> TD2ODIS_R {
        TD2ODIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&self) -> TE1ODIS_R {
        TE1ODIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&self) -> TE2ODIS_R {
        TE2ODIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta1odis(&mut self) -> TA1ODIS_W<DISR_SPEC, 0> {
        TA1ODIS_W::new(self)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn ta2odis(&mut self) -> TA2ODIS_W<DISR_SPEC, 1> {
        TA2ODIS_W::new(self)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb1odis(&mut self) -> TB1ODIS_W<DISR_SPEC, 2> {
        TB1ODIS_W::new(self)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tb2odis(&mut self) -> TB2ODIS_W<DISR_SPEC, 3> {
        TB2ODIS_W::new(self)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc1odis(&mut self) -> TC1ODIS_W<DISR_SPEC, 4> {
        TC1ODIS_W::new(self)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn tc2odis(&mut self) -> TC2ODIS_W<DISR_SPEC, 5> {
        TC2ODIS_W::new(self)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td1odis(&mut self) -> TD1ODIS_W<DISR_SPEC, 6> {
        TD1ODIS_W::new(self)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn td2odis(&mut self) -> TD2ODIS_W<DISR_SPEC, 7> {
        TD2ODIS_W::new(self)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te1odis(&mut self) -> TE1ODIS_W<DISR_SPEC, 8> {
        TE1ODIS_W::new(self)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    #[must_use]
    pub fn te2odis(&mut self) -> TE2ODIS_W<DISR_SPEC, 9> {
        TE2ODIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DISR_SPEC;
impl crate::RegisterSpec for DISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disr::R`](R) reader structure"]
impl crate::Readable for DISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`disr::W`](W) writer structure"]
impl crate::Writable for DISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DISR to value 0"]
impl crate::Resettable for DISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
