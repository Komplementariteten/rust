#[doc = "Register `MACTSCR` reader"]
pub type R = crate::R<MACTSCR_SPEC>;
#[doc = "Register `MACTSCR` writer"]
pub type W = crate::W<MACTSCR_SPEC>;
#[doc = "Field `TSENA` reader - TSENA"]
pub type TSENA_R = crate::BitReader;
#[doc = "Field `TSENA` writer - TSENA"]
pub type TSENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSCFUPDT` reader - TSCFUPDT"]
pub type TSCFUPDT_R = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - TSCFUPDT"]
pub type TSCFUPDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSINIT` reader - TSINIT"]
pub type TSINIT_R = crate::BitReader;
#[doc = "Field `TSINIT` writer - TSINIT"]
pub type TSINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSUPDT` reader - TSUPDT"]
pub type TSUPDT_R = crate::BitReader;
#[doc = "Field `TSUPDT` writer - TSUPDT"]
pub type TSUPDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSADDREG` reader - TSADDREG"]
pub type TSADDREG_R = crate::BitReader;
#[doc = "Field `TSADDREG` writer - TSADDREG"]
pub type TSADDREG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSENALL` reader - TSENALL"]
pub type TSENALL_R = crate::BitReader;
#[doc = "Field `TSENALL` writer - TSENALL"]
pub type TSENALL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSCTRLSSR` reader - TSCTRLSSR"]
pub type TSCTRLSSR_R = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - TSCTRLSSR"]
pub type TSCTRLSSR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSVER2ENA` reader - TSVER2ENA"]
pub type TSVER2ENA_R = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - TSVER2ENA"]
pub type TSVER2ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIPENA` reader - TSIPENA"]
pub type TSIPENA_R = crate::BitReader;
#[doc = "Field `TSIPENA` writer - TSIPENA"]
pub type TSIPENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIPV6ENA` reader - TSIPV6ENA"]
pub type TSIPV6ENA_R = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - TSIPV6ENA"]
pub type TSIPV6ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSIPV4ENA` reader - TSIPV4ENA"]
pub type TSIPV4ENA_R = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - TSIPV4ENA"]
pub type TSIPV4ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSEVNTENA` reader - TSEVNTENA"]
pub type TSEVNTENA_R = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - TSEVNTENA"]
pub type TSEVNTENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TSMSTRENA` reader - TSMSTRENA"]
pub type TSMSTRENA_R = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - TSMSTRENA"]
pub type TSMSTRENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SNAPTYPSEL` reader - SNAPTYPSEL"]
pub type SNAPTYPSEL_R = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - SNAPTYPSEL"]
pub type SNAPTYPSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TSENMACADDR` reader - TSENMACADDR"]
pub type TSENMACADDR_R = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - TSENMACADDR"]
pub type TSENMACADDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSC` reader - CSC"]
pub type CSC_R = crate::BitReader;
#[doc = "Field `TXTSSTSM` reader - TXTSSTSM"]
pub type TXTSSTSM_R = crate::BitReader;
#[doc = "Field `TXTSSTSM` writer - TXTSSTSM"]
pub type TXTSSTSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&self) -> TSENA_R {
        TSENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TSCFUPDT_R {
        TSCFUPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&self) -> TSINIT_R {
        TSINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TSUPDT_R {
        TSUPDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TSADDREG_R {
        TSADDREG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&self) -> TSENALL_R {
        TSENALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TSCTRLSSR_R {
        TSCTRLSSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> TSVER2ENA_R {
        TSVER2ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&self) -> TSIPENA_R {
        TSIPENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> TSIPV6ENA_R {
        TSIPV6ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> TSIPV4ENA_R {
        TSIPV4ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TSEVNTENA_R {
        TSEVNTENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TSMSTRENA_R {
        TSMSTRENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SNAPTYPSEL_R {
        SNAPTYPSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TSENMACADDR_R {
        TSENMACADDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&self) -> TXTSSTSM_R {
        TXTSSTSM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsena(&mut self) -> TSENA_W<MACTSCR_SPEC, 0> {
        TSENA_W::new(self)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    #[must_use]
    pub fn tscfupdt(&mut self) -> TSCFUPDT_W<MACTSCR_SPEC, 1> {
        TSCFUPDT_W::new(self)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    #[must_use]
    pub fn tsinit(&mut self) -> TSINIT_W<MACTSCR_SPEC, 2> {
        TSINIT_W::new(self)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    #[must_use]
    pub fn tsupdt(&mut self) -> TSUPDT_W<MACTSCR_SPEC, 3> {
        TSUPDT_W::new(self)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    #[must_use]
    pub fn tsaddreg(&mut self) -> TSADDREG_W<MACTSCR_SPEC, 5> {
        TSADDREG_W::new(self)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    #[must_use]
    pub fn tsenall(&mut self) -> TSENALL_W<MACTSCR_SPEC, 8> {
        TSENALL_W::new(self)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    #[must_use]
    pub fn tsctrlssr(&mut self) -> TSCTRLSSR_W<MACTSCR_SPEC, 9> {
        TSCTRLSSR_W::new(self)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsver2ena(&mut self) -> TSVER2ENA_W<MACTSCR_SPEC, 10> {
        TSVER2ENA_W::new(self)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipena(&mut self) -> TSIPENA_W<MACTSCR_SPEC, 11> {
        TSIPENA_W::new(self)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv6ena(&mut self) -> TSIPV6ENA_W<MACTSCR_SPEC, 12> {
        TSIPV6ENA_W::new(self)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsipv4ena(&mut self) -> TSIPV4ENA_W<MACTSCR_SPEC, 13> {
        TSIPV4ENA_W::new(self)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsevntena(&mut self) -> TSEVNTENA_W<MACTSCR_SPEC, 14> {
        TSEVNTENA_W::new(self)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    #[must_use]
    pub fn tsmstrena(&mut self) -> TSMSTRENA_W<MACTSCR_SPEC, 15> {
        TSMSTRENA_W::new(self)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn snaptypsel(&mut self) -> SNAPTYPSEL_W<MACTSCR_SPEC, 16> {
        SNAPTYPSEL_W::new(self)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    #[must_use]
    pub fn tsenmacaddr(&mut self) -> TSENMACADDR_W<MACTSCR_SPEC, 18> {
        TSENMACADDR_W::new(self)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    #[must_use]
    pub fn txtsstsm(&mut self) -> TXTSSTSM_W<MACTSCR_SPEC, 24> {
        TXTSSTSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timestamp control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSCR_SPEC;
impl crate::RegisterSpec for MACTSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactscr::R`](R) reader structure"]
impl crate::Readable for MACTSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mactscr::W`](W) writer structure"]
impl crate::Writable for MACTSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACTSCR to value 0x0200"]
impl crate::Resettable for MACTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
