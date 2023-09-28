#[doc = "Register `AXI_TARG4_FN_MOD_ISS_BM` reader"]
pub type R = crate::R<AXI_TARG4_FN_MOD_ISS_BM_SPEC>;
#[doc = "Register `AXI_TARG4_FN_MOD_ISS_BM` writer"]
pub type W = crate::W<AXI_TARG4_FN_MOD_ISS_BM_SPEC>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE"]
pub type READ_ISS_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Switch matrix write issuing override for target"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Switch matrix write issuing override for target"]
pub type WRITE_ISS_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Switch matrix write issuing override for target"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<AXI_TARG4_FN_MOD_ISS_BM_SPEC, 0> {
        READ_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Switch matrix write issuing override for target"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<AXI_TARG4_FN_MOD_ISS_BM_SPEC, 1> {
        WRITE_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_targ4_fn_mod_iss_bm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_targ4_fn_mod_iss_bm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_TARG4_FN_MOD_ISS_BM_SPEC;
impl crate::RegisterSpec for AXI_TARG4_FN_MOD_ISS_BM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ4_fn_mod_iss_bm::R`](R) reader structure"]
impl crate::Readable for AXI_TARG4_FN_MOD_ISS_BM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_targ4_fn_mod_iss_bm::W`](W) writer structure"]
impl crate::Writable for AXI_TARG4_FN_MOD_ISS_BM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_TARG4_FN_MOD_ISS_BM to value 0x04"]
impl crate::Resettable for AXI_TARG4_FN_MOD_ISS_BM_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
