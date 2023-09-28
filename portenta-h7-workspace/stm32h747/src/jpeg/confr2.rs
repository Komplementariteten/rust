#[doc = "Register `CONFR2` reader"]
pub type R = crate::R<CONFR2_SPEC>;
#[doc = "Register `CONFR2` writer"]
pub type W = crate::W<CONFR2_SPEC>;
#[doc = "Field `NMCU` reader - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
pub type NMCU_R = crate::FieldReader<u32>;
#[doc = "Field `NMCU` writer - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
pub type NMCU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 26, O, u32>;
impl R {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    pub fn nmcu(&self) -> NMCU_R {
        NMCU_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Number of MCU For encoding: this field defines the number of MCU units minus 1 to encode. For decoding: this field indicates the number of complete MCU units minus 1 to be decoded (this field is updated after the JPEG header parsing). If the decoded image size has not a X or Y size multiple of 8 or 16 (depending on the sub-sampling process), the resulting incomplete or empty MCU must be added to this value to get the total number of MCU generated."]
    #[inline(always)]
    #[must_use]
    pub fn nmcu(&mut self) -> NMCU_W<CONFR2_SPEC, 0> {
        NMCU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "JPEG codec configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFR2_SPEC;
impl crate::RegisterSpec for CONFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr2::R`](R) reader structure"]
impl crate::Readable for CONFR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`confr2::W`](W) writer structure"]
impl crate::Writable for CONFR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFR2 to value 0"]
impl crate::Resettable for CONFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
