#[doc = "Register `AXI_INI5_READ_QOS` reader"]
pub type R = crate::R<AXI_INI5_READ_QOS_SPEC>;
#[doc = "Register `AXI_INI5_READ_QOS` writer"]
pub type W = crate::W<AXI_INI5_READ_QOS_SPEC>;
#[doc = "Field `AR_QOS` reader - Read channel QoS setting"]
pub type AR_QOS_R = crate::FieldReader;
#[doc = "Field `AR_QOS` writer - Read channel QoS setting"]
pub type AR_QOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    pub fn ar_qos(&self) -> AR_QOS_R {
        AR_QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    #[must_use]
    pub fn ar_qos(&mut self) -> AR_QOS_W<AXI_INI5_READ_QOS_SPEC, 0> {
        AR_QOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_ini5_read_qos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_ini5_read_qos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_INI5_READ_QOS_SPEC;
impl crate::RegisterSpec for AXI_INI5_READ_QOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini5_read_qos::R`](R) reader structure"]
impl crate::Readable for AXI_INI5_READ_QOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_ini5_read_qos::W`](W) writer structure"]
impl crate::Writable for AXI_INI5_READ_QOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_INI5_READ_QOS to value 0x04"]
impl crate::Resettable for AXI_INI5_READ_QOS_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
