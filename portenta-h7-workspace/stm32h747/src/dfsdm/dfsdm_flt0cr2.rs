#[doc = "Register `DFSDM_FLT0CR2` reader"]
pub type R = crate::R<DFSDM_FLT0CR2_SPEC>;
#[doc = "Register `DFSDM_FLT0CR2` writer"]
pub type W = crate::W<DFSDM_FLT0CR2_SPEC>;
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub type JEOCIE_R = crate::BitReader;
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub type JEOCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub type REOCIE_R = crate::BitReader;
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub type REOCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub type JOVRIE_R = crate::BitReader;
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub type JOVRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub type ROVRIE_R = crate::BitReader;
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub type ROVRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AWDIE_R = crate::BitReader;
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AWDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCDIE` reader - Short-circuit detector interrupt enable"]
pub type SCDIE_R = crate::BitReader;
#[doc = "Field `SCDIE` writer - Short-circuit detector interrupt enable"]
pub type SCDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKABIE` reader - Clock absence interrupt enable"]
pub type CKABIE_R = crate::BitReader;
#[doc = "Field `CKABIE` writer - Clock absence interrupt enable"]
pub type CKABIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXCH` reader - Extremes detector channel selection"]
pub type EXCH_R = crate::FieldReader;
#[doc = "Field `EXCH` writer - Extremes detector channel selection"]
pub type EXCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub type AWDCH_R = crate::FieldReader;
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub type AWDCH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<DFSDM_FLT0CR2_SPEC, 0> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn reocie(&mut self) -> REOCIE_W<DFSDM_FLT0CR2_SPEC, 1> {
        REOCIE_W::new(self)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn jovrie(&mut self) -> JOVRIE_W<DFSDM_FLT0CR2_SPEC, 2> {
        JOVRIE_W::new(self)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rovrie(&mut self) -> ROVRIE_W<DFSDM_FLT0CR2_SPEC, 3> {
        ROVRIE_W::new(self)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<DFSDM_FLT0CR2_SPEC, 4> {
        AWDIE_W::new(self)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<DFSDM_FLT0CR2_SPEC, 5> {
        SCDIE_W::new(self)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<DFSDM_FLT0CR2_SPEC, 6> {
        CKABIE_W::new(self)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn exch(&mut self) -> EXCH_W<DFSDM_FLT0CR2_SPEC, 8> {
        EXCH_W::new(self)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<DFSDM_FLT0CR2_SPEC, 16> {
        AWDCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0CR2_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0cr2::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0cr2::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT0CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT0CR2 to value 0"]
impl crate::Resettable for DFSDM_FLT0CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
