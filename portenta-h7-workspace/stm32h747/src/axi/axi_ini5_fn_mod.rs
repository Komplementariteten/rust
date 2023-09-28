#[doc = "Register `AXI_INI5_FN_MOD` reader"]
pub type R = crate::R<AXI_INI5_FN_MOD_SPEC>;
#[doc = "Register `AXI_INI5_FN_MOD` writer"]
pub type W = crate::W<AXI_INI5_FN_MOD_SPEC>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability"]
pub type READ_ISS_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<AXI_INI5_FN_MOD_SPEC, 0> {
        READ_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<AXI_INI5_FN_MOD_SPEC, 1> {
        WRITE_ISS_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini5_fn_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini5_fn_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_INI5_FN_MOD_SPEC;
impl crate::RegisterSpec for AXI_INI5_FN_MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini5_fn_mod::R`](R) reader structure"]
impl crate::Readable for AXI_INI5_FN_MOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_ini5_fn_mod::W`](W) writer structure"]
impl crate::Writable for AXI_INI5_FN_MOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_INI5_FN_MOD to value 0x04"]
impl crate::Resettable for AXI_INI5_FN_MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
