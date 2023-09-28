#[doc = "Register `LWR` reader"]
pub type R = crate::R<LWR_SPEC>;
#[doc = "Register `LWR` writer"]
pub type W = crate::W<LWR_SPEC>;
#[doc = "Field `LW` reader - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LW_R = crate::FieldReader<u16>;
#[doc = "Field `LW` writer - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LW_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lw(&self) -> LW_R {
        LW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Line watermark These bits allow to configure the line watermark for interrupt generation. An interrupt is raised when the last pixel of the watermarked line has been transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn lw(&mut self) -> LW_W<LWR_SPEC, 0> {
        LW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA2D line watermark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LWR_SPEC;
impl crate::RegisterSpec for LWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lwr::R`](R) reader structure"]
impl crate::Readable for LWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lwr::W`](W) writer structure"]
impl crate::Writable for LWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LWR to value 0"]
impl crate::Resettable for LWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
