#[doc = "Register `SSCR` reader"]
pub type R = crate::R<SSCR_SPEC>;
#[doc = "Register `SSCR` writer"]
pub type W = crate::W<SSCR_SPEC>;
#[doc = "Field `VSH` reader - Vertical Synchronization Height (in units of horizontal scan line)"]
pub type VSH_R = crate::FieldReader<u16>;
#[doc = "Field `VSH` writer - Vertical Synchronization Height (in units of horizontal scan line)"]
pub type VSH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `HSW` reader - Horizontal Synchronization Width (in units of pixel clock period)"]
pub type HSW_R = crate::FieldReader<u16>;
#[doc = "Field `HSW` writer - Horizontal Synchronization Width (in units of pixel clock period)"]
pub type HSW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Synchronization Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)"]
    #[inline(always)]
    #[must_use]
    pub fn vsh(&mut self) -> VSH_W<SSCR_SPEC, 0> {
        VSH_W::new(self)
    }
    #[doc = "Bits 16:25 - Horizontal Synchronization Width (in units of pixel clock period)"]
    #[inline(always)]
    #[must_use]
    pub fn hsw(&mut self) -> HSW_W<SSCR_SPEC, 16> {
        HSW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Synchronization Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSCR_SPEC;
impl crate::RegisterSpec for SSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sscr::R`](R) reader structure"]
impl crate::Readable for SSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sscr::W`](W) writer structure"]
impl crate::Writable for SSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSCR to value 0"]
impl crate::Resettable for SSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
