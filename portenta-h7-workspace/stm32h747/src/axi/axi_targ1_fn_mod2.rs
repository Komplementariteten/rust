#[doc = "Register `AXI_TARG1_FN_MOD2` reader"]
pub type R = crate::R<AXI_TARG1_FN_MOD2_SPEC>;
#[doc = "Register `AXI_TARG1_FN_MOD2` writer"]
pub type W = crate::W<AXI_TARG1_FN_MOD2_SPEC>;
#[doc = "Field `BYPASS_MERGE` reader - Disable packing of beats to match the output data width"]
pub type BYPASS_MERGE_R = crate::BitReader;
#[doc = "Field `BYPASS_MERGE` writer - Disable packing of beats to match the output data width"]
pub type BYPASS_MERGE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Disable packing of beats to match the output data width"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable packing of beats to match the output data width"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W<AXI_TARG1_FN_MOD2_SPEC, 0> {
        BYPASS_MERGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ1_fn_mod2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ1_fn_mod2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_TARG1_FN_MOD2_SPEC;
impl crate::RegisterSpec for AXI_TARG1_FN_MOD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ1_fn_mod2::R`](R) reader structure"]
impl crate::Readable for AXI_TARG1_FN_MOD2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_targ1_fn_mod2::W`](W) writer structure"]
impl crate::Writable for AXI_TARG1_FN_MOD2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_TARG1_FN_MOD2 to value 0x04"]
impl crate::Resettable for AXI_TARG1_FN_MOD2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
