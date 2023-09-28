#[doc = "Register `CWSIZE` reader"]
pub type R = crate::R<CWSIZE_SPEC>;
#[doc = "Register `CWSIZE` writer"]
pub type W = crate::W<CWSIZE_SPEC>;
#[doc = "Field `CAPCNT` reader - Capture count"]
pub type CAPCNT_R = crate::FieldReader<u16>;
#[doc = "Field `CAPCNT` writer - Capture count"]
pub type CAPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `VLINE` reader - Vertical line count"]
pub type VLINE_R = crate::FieldReader<u16>;
#[doc = "Field `VLINE` writer - Vertical line count"]
pub type VLINE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    pub fn capcnt(&self) -> CAPCNT_R {
        CAPCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    pub fn vline(&self) -> VLINE_R {
        VLINE_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Capture count"]
    #[inline(always)]
    #[must_use]
    pub fn capcnt(&mut self) -> CAPCNT_W<CWSIZE_SPEC, 0> {
        CAPCNT_W::new(self)
    }
    #[doc = "Bits 16:29 - Vertical line count"]
    #[inline(always)]
    #[must_use]
    pub fn vline(&mut self) -> VLINE_W<CWSIZE_SPEC, 16> {
        VLINE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "crop window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSIZE_SPEC;
impl crate::RegisterSpec for CWSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwsize::R`](R) reader structure"]
impl crate::Readable for CWSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwsize::W`](W) writer structure"]
impl crate::Writable for CWSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CWSIZE to value 0"]
impl crate::Resettable for CWSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
