#[doc = "Register `CONFR3` reader"]
pub type R = crate::R<CONFR3_SPEC>;
#[doc = "Register `CONFR3` writer"]
pub type W = crate::W<CONFR3_SPEC>;
#[doc = "Field `XSIZE` reader - X size This field defines the number of pixels per line."]
pub type XSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `XSIZE` writer - X size This field defines the number of pixels per line."]
pub type XSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - X size This field defines the number of pixels per line."]
    #[inline(always)]
    pub fn xsize(&self) -> XSIZE_R {
        XSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - X size This field defines the number of pixels per line."]
    #[inline(always)]
    #[must_use]
    pub fn xsize(&mut self) -> XSIZE_W<CONFR3_SPEC, 16> {
        XSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "JPEG codec configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFR3_SPEC;
impl crate::RegisterSpec for CONFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr3::R`](R) reader structure"]
impl crate::Readable for CONFR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`confr3::W`](W) writer structure"]
impl crate::Writable for CONFR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFR3 to value 0"]
impl crate::Resettable for CONFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
