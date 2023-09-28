#[doc = "Register `AXI_INI3_FN_MOD_AHB` reader"]
pub type R = crate::R<AXI_INI3_FN_MOD_AHB_SPEC>;
#[doc = "Register `AXI_INI3_FN_MOD_AHB` writer"]
pub type W = crate::W<AXI_INI3_FN_MOD_AHB_SPEC>;
#[doc = "Field `RD_INC_OVERRIDE` reader - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RD_INC_OVERRIDE_R = crate::BitReader;
#[doc = "Field `RD_INC_OVERRIDE` writer - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RD_INC_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WR_INC_OVERRIDE` reader - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WR_INC_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WR_INC_OVERRIDE` writer - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WR_INC_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RD_INC_OVERRIDE_R {
        RD_INC_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WR_INC_OVERRIDE_R {
        WR_INC_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    #[must_use]
    pub fn rd_inc_override(&mut self) -> RD_INC_OVERRIDE_W<AXI_INI3_FN_MOD_AHB_SPEC, 0> {
        RD_INC_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    #[must_use]
    pub fn wr_inc_override(&mut self) -> WR_INC_OVERRIDE_W<AXI_INI3_FN_MOD_AHB_SPEC, 1> {
        WR_INC_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini3_fn_mod_ahb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini3_fn_mod_ahb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_INI3_FN_MOD_AHB_SPEC;
impl crate::RegisterSpec for AXI_INI3_FN_MOD_AHB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini3_fn_mod_ahb::R`](R) reader structure"]
impl crate::Readable for AXI_INI3_FN_MOD_AHB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_ini3_fn_mod_ahb::W`](W) writer structure"]
impl crate::Writable for AXI_INI3_FN_MOD_AHB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_INI3_FN_MOD_AHB to value 0x04"]
impl crate::Resettable for AXI_INI3_FN_MOD_AHB_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
