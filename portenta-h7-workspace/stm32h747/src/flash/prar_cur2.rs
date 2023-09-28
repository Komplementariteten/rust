#[doc = "Register `PRAR_CUR2` reader"]
pub type R = crate::R<PRAR_CUR2_SPEC>;
#[doc = "Field `PROT_AREA_START2` reader - Bank 2 lowest PCROP protected address"]
pub type PROT_AREA_START2_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END2` reader - Bank 2 highest PCROP protected address"]
pub type PROT_AREA_END2_R = crate::FieldReader<u16>;
#[doc = "Field `DMEP2` reader - Bank 2 PCROP protected erase enable option status bit"]
pub type DMEP2_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Bank 2 lowest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_start2(&self) -> PROT_AREA_START2_R {
        PROT_AREA_START2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 2 highest PCROP protected address"]
    #[inline(always)]
    pub fn prot_area_end2(&self) -> PROT_AREA_END2_R {
        PROT_AREA_END2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 2 PCROP protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmep2(&self) -> DMEP2_R {
        DMEP2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_cur2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRAR_CUR2_SPEC;
impl crate::RegisterSpec for PRAR_CUR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_cur2::R`](R) reader structure"]
impl crate::Readable for PRAR_CUR2_SPEC {}
#[doc = "`reset()` method sets PRAR_CUR2 to value 0"]
impl crate::Resettable for PRAR_CUR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
