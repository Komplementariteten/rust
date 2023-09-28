#[doc = "Register `SCAR_CUR2` reader"]
pub type R = crate::R<SCAR_CUR2_SPEC>;
#[doc = "Register `SCAR_CUR2` writer"]
pub type W = crate::W<SCAR_CUR2_SPEC>;
#[doc = "Field `SEC_AREA_START2` reader - Bank 2 lowest secure protected address"]
pub type SEC_AREA_START2_R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_START2` writer - Bank 2 lowest secure protected address"]
pub type SEC_AREA_START2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `SEC_AREA_END2` reader - Bank 2 highest secure protected address"]
pub type SEC_AREA_END2_R = crate::FieldReader<u16>;
#[doc = "Field `SEC_AREA_END2` writer - Bank 2 highest secure protected address"]
pub type SEC_AREA_END2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DMES2` reader - Bank 2 secure protected erase enable option status bit"]
pub type DMES2_R = crate::BitReader;
#[doc = "Field `DMES2` writer - Bank 2 secure protected erase enable option status bit"]
pub type DMES2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Bank 2 lowest secure protected address"]
    #[inline(always)]
    pub fn sec_area_start2(&self) -> SEC_AREA_START2_R {
        SEC_AREA_START2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 2 highest secure protected address"]
    #[inline(always)]
    pub fn sec_area_end2(&self) -> SEC_AREA_END2_R {
        SEC_AREA_END2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 2 secure protected erase enable option status bit"]
    #[inline(always)]
    pub fn dmes2(&self) -> DMES2_R {
        DMES2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 2 lowest secure protected address"]
    #[inline(always)]
    #[must_use]
    pub fn sec_area_start2(&mut self) -> SEC_AREA_START2_W<SCAR_CUR2_SPEC, 0> {
        SEC_AREA_START2_W::new(self)
    }
    #[doc = "Bits 16:27 - Bank 2 highest secure protected address"]
    #[inline(always)]
    #[must_use]
    pub fn sec_area_end2(&mut self) -> SEC_AREA_END2_W<SCAR_CUR2_SPEC, 16> {
        SEC_AREA_END2_W::new(self)
    }
    #[doc = "Bit 31 - Bank 2 secure protected erase enable option status bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmes2(&mut self) -> DMES2_W<SCAR_CUR2_SPEC, 31> {
        DMES2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH secure address for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scar_cur2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scar_cur2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCAR_CUR2_SPEC;
impl crate::RegisterSpec for SCAR_CUR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scar_cur2::R`](R) reader structure"]
impl crate::Readable for SCAR_CUR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scar_cur2::W`](W) writer structure"]
impl crate::Writable for SCAR_CUR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCAR_CUR2 to value 0"]
impl crate::Resettable for SCAR_CUR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
