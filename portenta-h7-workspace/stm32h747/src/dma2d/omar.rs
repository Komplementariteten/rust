#[doc = "Register `OMAR` reader"]
pub type R = crate::R<OMAR_SPEC>;
#[doc = "Register `OMAR` writer"]
pub type W = crate::W<OMAR_SPEC>;
#[doc = "Field `MA` reader - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
pub type MA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address Address of the data used for the output FIFO. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned and a 16-bit per pixel format must be 16-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<OMAR_SPEC, 0> {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA2D output memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OMAR_SPEC;
impl crate::RegisterSpec for OMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`omar::R`](R) reader structure"]
impl crate::Readable for OMAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`omar::W`](W) writer structure"]
impl crate::Writable for OMAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OMAR to value 0"]
impl crate::Resettable for OMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
