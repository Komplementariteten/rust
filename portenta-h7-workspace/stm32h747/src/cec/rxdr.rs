#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RXDR_SPEC>;
#[doc = "Field `RXD` reader - Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line."]
pub type RXD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Rx Data register. RXD is read-only and contains the last data byte which has been received from the CEC line."]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CEC Rx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDR_SPEC;
impl crate::RegisterSpec for RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RXDR_SPEC {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
