#[doc = "Register `OOR` reader"]
pub type R = crate::R<OOR_SPEC>;
#[doc = "Register `OOR` writer"]
pub type W = crate::W<OOR_SPEC>;
#[doc = "Field `LO` reader - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LO_R = crate::FieldReader<u16>;
#[doc = "Field `LO` writer - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
pub type LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line Offset Line offset used for the output (expressed in pixels). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<OOR_SPEC, 0> {
        LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA2D output offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OOR_SPEC;
impl crate::RegisterSpec for OOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oor::R`](R) reader structure"]
impl crate::Readable for OOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oor::W`](W) writer structure"]
impl crate::Writable for OOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OOR to value 0"]
impl crate::Resettable for OOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
