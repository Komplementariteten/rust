#[doc = "Register `OTG_HS_DIEPTXF0_Device` reader"]
pub type R = crate::R<OTG_HS_DIEPTXF0_DEVICE_SPEC>;
#[doc = "Register `OTG_HS_DIEPTXF0_Device` writer"]
pub type W = crate::W<OTG_HS_DIEPTXF0_DEVICE_SPEC>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<OTG_HS_DIEPTXF0_DEVICE_SPEC, 0> {
        TX0FSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<OTG_HS_DIEPTXF0_DEVICE_SPEC, 16> {
        TX0FD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_dieptxf0_device::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_dieptxf0_device::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HS_DIEPTXF0_DEVICE_SPEC;
impl crate::RegisterSpec for OTG_HS_DIEPTXF0_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dieptxf0_device::R`](R) reader structure"]
impl crate::Readable for OTG_HS_DIEPTXF0_DEVICE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dieptxf0_device::W`](W) writer structure"]
impl crate::Writable for OTG_HS_DIEPTXF0_DEVICE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DIEPTXF0_Device to value 0x0200"]
impl crate::Resettable for OTG_HS_DIEPTXF0_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
