#[doc = "Register `FLTDR` reader"]
pub type R = crate::R<FLTDR_SPEC>;
#[doc = "Register `FLTDR` writer"]
pub type W = crate::W<FLTDR_SPEC>;
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type FLT1EN_R = crate::BitReader;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type FLT1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type FLT2EN_R = crate::BitReader;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type FLT2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type FLT3EN_R = crate::BitReader;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type FLT3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type FLT4EN_R = crate::BitReader;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type FLT4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type FLT5EN_R = crate::BitReader;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub type FLT5EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FLTLCK_R = crate::BitReader;
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FLTLCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<FLTDR_SPEC, 0> {
        FLT1EN_W::new(self)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<FLTDR_SPEC, 1> {
        FLT2EN_W::new(self)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<FLTDR_SPEC, 2> {
        FLT3EN_W::new(self)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<FLTDR_SPEC, 3> {
        FLT4EN_W::new(self)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<FLTDR_SPEC, 4> {
        FLT5EN_W::new(self)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<FLTDR_SPEC, 31> {
        FLTLCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timerx Fault Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fltdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fltdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLTDR_SPEC;
impl crate::RegisterSpec for FLTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltdr::R`](R) reader structure"]
impl crate::Readable for FLTDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fltdr::W`](W) writer structure"]
impl crate::Writable for FLTDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLTDR to value 0"]
impl crate::Resettable for FLTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
