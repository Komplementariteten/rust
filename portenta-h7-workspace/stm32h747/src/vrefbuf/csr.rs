#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `ENVR` reader - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
pub type ENVR_R = crate::BitReader;
#[doc = "Field `ENVR` writer - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
pub type ENVR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIZ` reader - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
pub type HIZ_R = crate::BitReader;
#[doc = "Field `HIZ` writer - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
pub type HIZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VRR` reader - Voltage reference buffer ready"]
pub type VRR_R = crate::BitReader;
#[doc = "Field `VRS` reader - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
pub type VRS_R = crate::FieldReader;
#[doc = "Field `VRS` writer - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
pub type VRS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage reference buffer ready"]
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage reference buffer mode enable This bit is used to enable the voltage reference buffer mode."]
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> ENVR_W<CSR_SPEC, 0> {
        ENVR_W::new(self)
    }
    #[doc = "Bit 1 - High impedance mode This bit controls the analog switch to connect or not the VREF+ pin. Refer to Table196: VREF buffer modes for the mode descriptions depending on ENVR bit configuration."]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<CSR_SPEC, 1> {
        HIZ_W::new(self)
    }
    #[doc = "Bits 4:6 - Voltage reference scale These bits select the value generated by the voltage reference buffer. Other: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VRS_W<CSR_SPEC, 4> {
        VRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "VREFBUF control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x02"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}