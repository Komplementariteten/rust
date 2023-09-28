#[doc = "Register `AHBSCR` reader"]
pub type R = crate::R<AHBSCR_SPEC>;
#[doc = "Register `AHBSCR` writer"]
pub type W = crate::W<AHBSCR_SPEC>;
#[doc = "Field `CTL` reader - CTL"]
pub type CTL_R = crate::FieldReader;
#[doc = "Field `CTL` writer - CTL"]
pub type CTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TPRI` reader - TPRI"]
pub type TPRI_R = crate::FieldReader<u16>;
#[doc = "Field `TPRI` writer - TPRI"]
pub type TPRI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `INITCOUNT` reader - INITCOUNT"]
pub type INITCOUNT_R = crate::FieldReader;
#[doc = "Field `INITCOUNT` writer - INITCOUNT"]
pub type INITCOUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:1 - CTL"]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - TPRI"]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - INITCOUNT"]
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTL"]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CTL_W<AHBSCR_SPEC, 0> {
        CTL_W::new(self)
    }
    #[doc = "Bits 2:10 - TPRI"]
    #[inline(always)]
    #[must_use]
    pub fn tpri(&mut self) -> TPRI_W<AHBSCR_SPEC, 2> {
        TPRI_W::new(self)
    }
    #[doc = "Bits 11:15 - INITCOUNT"]
    #[inline(always)]
    #[must_use]
    pub fn initcount(&mut self) -> INITCOUNT_W<AHBSCR_SPEC, 11> {
        INITCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB Slave Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBSCR_SPEC;
impl crate::RegisterSpec for AHBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbscr::R`](R) reader structure"]
impl crate::Readable for AHBSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbscr::W`](W) writer structure"]
impl crate::Writable for AHBSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBSCR to value 0"]
impl crate::Resettable for AHBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
