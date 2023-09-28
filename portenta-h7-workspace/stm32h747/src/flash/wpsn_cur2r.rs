#[doc = "Register `WPSN_CUR2R` reader"]
pub type R = crate::R<WPSN_CUR2R_SPEC>;
#[doc = "Field `WRPSn2` reader - Bank 2 sector write protection option status byte"]
pub type WRPSN2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bank 2 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn2(&self) -> WRPSN2_R {
        WRPSN2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_cur2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSN_CUR2R_SPEC;
impl crate::RegisterSpec for WPSN_CUR2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_cur2r::R`](R) reader structure"]
impl crate::Readable for WPSN_CUR2R_SPEC {}
#[doc = "`reset()` method sets WPSN_CUR2R to value 0"]
impl crate::Resettable for WPSN_CUR2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
