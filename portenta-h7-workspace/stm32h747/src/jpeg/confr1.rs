#[doc = "Register `CONFR1` reader"]
pub type R = crate::R<CONFR1_SPEC>;
#[doc = "Register `CONFR1` writer"]
pub type W = crate::W<CONFR1_SPEC>;
#[doc = "Field `NF` reader - Number of color components This field defines the number of color components minus 1."]
pub type NF_R = crate::FieldReader;
#[doc = "Field `NF` writer - Number of color components This field defines the number of color components minus 1."]
pub type NF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DE` reader - Decoding Enable This bit selects the coding or decoding process"]
pub type DE_R = crate::BitReader;
#[doc = "Field `DE` writer - Decoding Enable This bit selects the coding or decoding process"]
pub type DE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COLORSPACE` reader - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
pub type COLORSPACE_R = crate::FieldReader;
#[doc = "Field `COLORSPACE` writer - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
pub type COLORSPACE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `NS` reader - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
pub type NS_R = crate::FieldReader;
#[doc = "Field `NS` writer - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
pub type NS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `HDR` reader - Header Processing This bit enable the header processing (generation/parsing)."]
pub type HDR_R = crate::BitReader;
#[doc = "Field `HDR` writer - Header Processing This bit enable the header processing (generation/parsing)."]
pub type HDR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `YSIZE` reader - Y Size This field defines the number of lines in source image."]
pub type YSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `YSIZE` writer - Y Size This field defines the number of lines in source image."]
pub type YSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:1 - Number of color components This field defines the number of color components minus 1."]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Decoding Enable This bit selects the coding or decoding process"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
    #[inline(always)]
    pub fn colorspace(&self) -> COLORSPACE_R {
        COLORSPACE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Header Processing This bit enable the header processing (generation/parsing)."]
    #[inline(always)]
    pub fn hdr(&self) -> HDR_R {
        HDR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Y Size This field defines the number of lines in source image."]
    #[inline(always)]
    pub fn ysize(&self) -> YSIZE_R {
        YSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of color components This field defines the number of color components minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NF_W<CONFR1_SPEC, 0> {
        NF_W::new(self)
    }
    #[doc = "Bit 3 - Decoding Enable This bit selects the coding or decoding process"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<CONFR1_SPEC, 3> {
        DE_W::new(self)
    }
    #[doc = "Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
    #[inline(always)]
    #[must_use]
    pub fn colorspace(&mut self) -> COLORSPACE_W<CONFR1_SPEC, 4> {
        COLORSPACE_W::new(self)
    }
    #[doc = "Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<CONFR1_SPEC, 6> {
        NS_W::new(self)
    }
    #[doc = "Bit 8 - Header Processing This bit enable the header processing (generation/parsing)."]
    #[inline(always)]
    #[must_use]
    pub fn hdr(&mut self) -> HDR_W<CONFR1_SPEC, 8> {
        HDR_W::new(self)
    }
    #[doc = "Bits 16:31 - Y Size This field defines the number of lines in source image."]
    #[inline(always)]
    #[must_use]
    pub fn ysize(&mut self) -> YSIZE_W<CONFR1_SPEC, 16> {
        YSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "JPEG codec configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFR1_SPEC;
impl crate::RegisterSpec for CONFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr1::R`](R) reader structure"]
impl crate::Readable for CONFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`confr1::W`](W) writer structure"]
impl crate::Writable for CONFR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFR1 to value 0"]
impl crate::Resettable for CONFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
