#[doc = "Register `UR9` reader"]
pub type R = crate::R<UR9_SPEC>;
#[doc = "Field `WRPS_2` reader - Write protection for flash bank 2"]
pub type WRPS_2_R = crate::FieldReader;
#[doc = "Field `PA_BEG_2` reader - Protected area start address for bank 2"]
pub type PA_BEG_2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Write protection for flash bank 2"]
    #[inline(always)]
    pub fn wrps_2(&self) -> WRPS_2_R {
        WRPS_2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:27 - Protected area start address for bank 2"]
    #[inline(always)]
    pub fn pa_beg_2(&self) -> PA_BEG_2_R {
        PA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR9_SPEC;
impl crate::RegisterSpec for UR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur9::R`](R) reader structure"]
impl crate::Readable for UR9_SPEC {}
#[doc = "`reset()` method sets UR9 to value 0"]
impl crate::Resettable for UR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
