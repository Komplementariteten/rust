#[doc = "Register `TWCR` reader"]
pub type R = crate::R<TWCR_SPEC>;
#[doc = "Register `TWCR` writer"]
pub type W = crate::W<TWCR_SPEC>;
#[doc = "Field `TOTALH` reader - Total Height (in units of horizontal scan line)"]
pub type TOTALH_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALH` writer - Total Height (in units of horizontal scan line)"]
pub type TOTALH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `TOTALW` reader - Total Width (in units of pixel clock period)"]
pub type TOTALW_R = crate::FieldReader<u16>;
#[doc = "Field `TOTALW` writer - Total Width (in units of pixel clock period)"]
pub type TOTALW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn totalh(&self) -> TOTALH_R {
        TOTALH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    pub fn totalw(&self) -> TOTALW_R {
        TOTALW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Total Height (in units of horizontal scan line)"]
    #[inline(always)]
    #[must_use]
    pub fn totalh(&mut self) -> TOTALH_W<TWCR_SPEC, 0> {
        TOTALH_W::new(self)
    }
    #[doc = "Bits 16:27 - Total Width (in units of pixel clock period)"]
    #[inline(always)]
    #[must_use]
    pub fn totalw(&mut self) -> TOTALW_W<TWCR_SPEC, 16> {
        TOTALW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Total Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`twcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWCR_SPEC;
impl crate::RegisterSpec for TWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twcr::R`](R) reader structure"]
impl crate::Readable for TWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twcr::W`](W) writer structure"]
impl crate::Writable for TWCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWCR to value 0"]
impl crate::Resettable for TWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
