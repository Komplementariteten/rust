#[doc = "Register `AXI_TARG2_FN_MOD_LB` reader"]
pub type R = crate::R<AXI_TARG2_FN_MOD_LB_SPEC>;
#[doc = "Register `AXI_TARG2_FN_MOD_LB` writer"]
pub type W = crate::W<AXI_TARG2_FN_MOD_LB_SPEC>;
#[doc = "Field `FN_MOD_LB` reader - Controls burst breaking of long bursts"]
pub type FN_MOD_LB_R = crate::BitReader;
#[doc = "Field `FN_MOD_LB` writer - Controls burst breaking of long bursts"]
pub type FN_MOD_LB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FN_MOD_LB_R {
        FN_MOD_LB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    #[must_use]
    pub fn fn_mod_lb(&mut self) -> FN_MOD_LB_W<AXI_TARG2_FN_MOD_LB_SPEC, 0> {
        FN_MOD_LB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ2_fn_mod_lb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_lb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_TARG2_FN_MOD_LB_SPEC;
impl crate::RegisterSpec for AXI_TARG2_FN_MOD_LB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ2_fn_mod_lb::R`](R) reader structure"]
impl crate::Readable for AXI_TARG2_FN_MOD_LB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_targ2_fn_mod_lb::W`](W) writer structure"]
impl crate::Writable for AXI_TARG2_FN_MOD_LB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_TARG2_FN_MOD_LB to value 0x04"]
impl crate::Resettable for AXI_TARG2_FN_MOD_LB_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
