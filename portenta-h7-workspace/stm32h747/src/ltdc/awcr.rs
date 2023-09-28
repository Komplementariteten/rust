#[doc = "Register `AWCR` reader"]
pub type R = crate::R<AWCR_SPEC>;
#[doc = "Register `AWCR` writer"]
pub type W = crate::W<AWCR_SPEC>;
#[doc = "Field `AAH` reader - Accumulated Active Height (in units of horizontal scan line)"]
pub type AAH_R = crate::FieldReader<u16>;
#[doc = "Field `AAH` writer - Accumulated Active Height (in units of horizontal scan line)"]
pub type AAH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `AAV` reader - AAV"]
pub type AAV_R = crate::FieldReader<u16>;
#[doc = "Field `AAV` writer - AAV"]
pub type AAV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - AAV"]
    #[inline(always)]
    pub fn aav(&self) -> AAV_R {
        AAV_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)"]
    #[inline(always)]
    #[must_use]
    pub fn aah(&mut self) -> AAH_W<AWCR_SPEC, 0> {
        AAH_W::new(self)
    }
    #[doc = "Bits 16:27 - AAV"]
    #[inline(always)]
    #[must_use]
    pub fn aav(&mut self) -> AAV_W<AWCR_SPEC, 16> {
        AAV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Active Width Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWCR_SPEC;
impl crate::RegisterSpec for AWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awcr::R`](R) reader structure"]
impl crate::Readable for AWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awcr::W`](W) writer structure"]
impl crate::Writable for AWCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWCR to value 0"]
impl crate::Resettable for AWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
