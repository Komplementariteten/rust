#[doc = "Register `L2CLUTWR` writer"]
pub type W = crate::W<L2CLUTWR_SPEC>;
#[doc = "Field `BLUE` writer - Blue value"]
pub type BLUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `GREEN` writer - Green value"]
pub type GREEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RED` writer - Red value"]
pub type RED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLUTADD` writer - CLUT Address"]
pub type CLUTADD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Blue value"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<L2CLUTWR_SPEC, 0> {
        BLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<L2CLUTWR_SPEC, 8> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<L2CLUTWR_SPEC, 16> {
        RED_W::new(self)
    }
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<L2CLUTWR_SPEC, 24> {
        CLUTADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layerx CLUT Write Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CLUTWR_SPEC;
impl crate::RegisterSpec for L2CLUTWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`l2clutwr::W`](W) writer structure"]
impl crate::Writable for L2CLUTWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2CLUTWR to value 0"]
impl crate::Resettable for L2CLUTWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
