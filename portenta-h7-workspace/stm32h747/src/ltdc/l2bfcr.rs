#[doc = "Register `L2BFCR` reader"]
pub type R = crate::R<L2BFCR_SPEC>;
#[doc = "Register `L2BFCR` writer"]
pub type W = crate::W<L2BFCR_SPEC>;
#[doc = "Field `BF2` reader - Blending Factor 2"]
pub type BF2_R = crate::FieldReader;
#[doc = "Field `BF2` writer - Blending Factor 2"]
pub type BF2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BF1` reader - Blending Factor 1"]
pub type BF1_R = crate::FieldReader;
#[doc = "Field `BF1` writer - Blending Factor 1"]
pub type BF1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Blending Factor 2"]
    #[inline(always)]
    #[must_use]
    pub fn bf2(&mut self) -> BF2_W<L2BFCR_SPEC, 0> {
        BF2_W::new(self)
    }
    #[doc = "Bits 8:10 - Blending Factor 1"]
    #[inline(always)]
    #[must_use]
    pub fn bf1(&mut self) -> BF1_W<L2BFCR_SPEC, 8> {
        BF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layerx Blending Factors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2bfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2bfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2BFCR_SPEC;
impl crate::RegisterSpec for L2BFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2bfcr::R`](R) reader structure"]
impl crate::Readable for L2BFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2bfcr::W`](W) writer structure"]
impl crate::Writable for L2BFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2BFCR to value 0x0607"]
impl crate::Resettable for L2BFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0607;
}
