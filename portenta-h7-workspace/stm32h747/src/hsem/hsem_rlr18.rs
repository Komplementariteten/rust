#[doc = "Register `HSEM_RLR18` reader"]
pub type R = crate::R<HSEM_RLR18_SPEC>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `COREID` reader - Semaphore COREID"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LOCK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HSEM Read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr18::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_RLR18_SPEC;
impl crate::RegisterSpec for HSEM_RLR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_rlr18::R`](R) reader structure"]
impl crate::Readable for HSEM_RLR18_SPEC {}
#[doc = "`reset()` method sets HSEM_RLR18 to value 0"]
impl crate::Resettable for HSEM_RLR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
