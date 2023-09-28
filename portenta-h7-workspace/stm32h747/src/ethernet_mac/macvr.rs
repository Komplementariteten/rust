#[doc = "Register `MACVR` reader"]
pub type R = crate::R<MACVR_SPEC>;
#[doc = "Field `SNPSVER` reader - SNPSVER"]
pub type SNPSVER_R = crate::FieldReader;
#[doc = "Field `USERVER` reader - USERVER"]
pub type USERVER_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SNPSVER"]
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USERVER"]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVR_SPEC;
impl crate::RegisterSpec for MACVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvr::R`](R) reader structure"]
impl crate::Readable for MACVR_SPEC {}
#[doc = "`reset()` method sets MACVR to value 0x3041"]
impl crate::Resettable for MACVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3041;
}
