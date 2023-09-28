#[doc = "Register `PRAR_CUR1` reader"]
pub type R = crate::R<PRAR_CUR1_SPEC>;
#[doc = "Field `PROT_AREA_START1` reader - Bank 1 lowest PCROP protected address"]
pub type PROT_AREA_START1_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END1` reader - Bank 1 highest PCROP protected address"]
pub type PROT_AREA_END1_R = crate::FieldReader<u16>;
#[doc = "Field `DMEP1` reader - Bank 1 PCROP protected erase enable option status bit"]
pub type DMEP1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start1(&self) -> PROT_AREA_START1_R {
        PROT_AREA_START1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end1(&self) -> PROT_AREA_END1_R {
        PROT_AREA_END1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep1(&self) -> DMEP1_R {
        DMEP1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_cur1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRAR_CUR1_SPEC;
impl crate::RegisterSpec for PRAR_CUR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_cur1::R`](R) reader structure"]
impl crate::Readable for PRAR_CUR1_SPEC {}
#[doc = "`reset()` method sets PRAR_CUR1 to value 0"]
impl crate::Resettable for PRAR_CUR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
