#[doc = "Register `PRAR_PRG2` reader"]
pub type R = crate::R<PRAR_PRG2_SPEC>;
#[doc = "Register `PRAR_PRG2` writer"]
pub type W = crate::W<PRAR_PRG2_SPEC>;
#[doc = "Field `PROT_AREA_START2` reader - Bank 2 lowest PCROP protected address configuration"]
pub type PROT_AREA_START2_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_START2` writer - Bank 2 lowest PCROP protected address configuration"]
pub type PROT_AREA_START2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `PROT_AREA_END2` reader - Bank 2 highest PCROP protected address configuration"]
pub type PROT_AREA_END2_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END2` writer - Bank 2 highest PCROP protected address configuration"]
pub type PROT_AREA_END2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DMEP2` reader - Bank 2 PCROP protected erase enable option configuration bit"]
pub type DMEP2_R = crate::BitReader;
#[doc = "Field `DMEP2` writer - Bank 2 PCROP protected erase enable option configuration bit"]
pub type DMEP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Bank 2 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start2(&self) -> PROT_AREA_START2_R {
        PROT_AREA_START2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 2 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end2(&self) -> PROT_AREA_END2_R {
        PROT_AREA_END2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 2 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep2(&self) -> DMEP2_R {
        DMEP2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 2 lowest PCROP protected address configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_start2(&mut self) -> PROT_AREA_START2_W<PRAR_PRG2_SPEC, 0> {
        PROT_AREA_START2_W::new(self)
    }
    #[doc = "Bits 16:27 - Bank 2 highest PCROP protected address configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_end2(&mut self) -> PROT_AREA_END2_W<PRAR_PRG2_SPEC, 16> {
        PROT_AREA_END2_W::new(self)
    }
    #[doc = "Bit 31 - Bank 2 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmep2(&mut self) -> DMEP2_W<PRAR_PRG2_SPEC, 31> {
        DMEP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH protection address for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_prg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prar_prg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRAR_PRG2_SPEC;
impl crate::RegisterSpec for PRAR_PRG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_prg2::R`](R) reader structure"]
impl crate::Readable for PRAR_PRG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prar_prg2::W`](W) writer structure"]
impl crate::Writable for PRAR_PRG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRAR_PRG2 to value 0"]
impl crate::Resettable for PRAR_PRG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
