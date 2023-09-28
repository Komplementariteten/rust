#[doc = "Register `NLR` reader"]
pub type R = crate::R<NLR_SPEC>;
#[doc = "Register `NLR` writer"]
pub type W = crate::W<NLR_SPEC>;
#[doc = "Field `NL` reader - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type NL_R = crate::FieldReader<u16>;
#[doc = "Field `NL` writer - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type NL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `PL` reader - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
pub type PL_R = crate::FieldReader<u16>;
#[doc = "Field `PL` writer - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
pub type PL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn nl(&mut self) -> NL_W<NLR_SPEC, 0> {
        NL_W::new(self)
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<NLR_SPEC, 16> {
        PL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA2D number of line register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NLR_SPEC;
impl crate::RegisterSpec for NLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nlr::R`](R) reader structure"]
impl crate::Readable for NLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nlr::W`](W) writer structure"]
impl crate::Writable for NLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NLR to value 0"]
impl crate::Resettable for NLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
