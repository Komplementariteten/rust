#[doc = "Register `PRAR_PRG1` reader"]
pub type R = crate::R<PRAR_PRG1_SPEC>;
#[doc = "Register `PRAR_PRG1` writer"]
pub type W = crate::W<PRAR_PRG1_SPEC>;
#[doc = "Field `PROT_AREA_START1` reader - Bank 1 lowest PCROP protected address configuration"]
pub type PROT_AREA_START1_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_START1` writer - Bank 1 lowest PCROP protected address configuration"]
pub type PROT_AREA_START1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `PROT_AREA_END1` reader - Bank 1 highest PCROP protected address configuration"]
pub type PROT_AREA_END1_R = crate::FieldReader<u16>;
#[doc = "Field `PROT_AREA_END1` writer - Bank 1 highest PCROP protected address configuration"]
pub type PROT_AREA_END1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DMEP1` reader - Bank 1 PCROP protected erase enable option configuration bit"]
pub type DMEP1_R = crate::BitReader;
#[doc = "Field `DMEP1` writer - Bank 1 PCROP protected erase enable option configuration bit"]
pub type DMEP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_start1(&self) -> PROT_AREA_START1_R {
        PROT_AREA_START1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address configuration"]
    #[inline(always)]
    pub fn prot_area_end1(&self) -> PROT_AREA_END1_R {
        PROT_AREA_END1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    pub fn dmep1(&self) -> DMEP1_R {
        DMEP1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bank 1 lowest PCROP protected address configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_start1(&mut self) -> PROT_AREA_START1_W<PRAR_PRG1_SPEC, 0> {
        PROT_AREA_START1_W::new(self)
    }
    #[doc = "Bits 16:27 - Bank 1 highest PCROP protected address configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prot_area_end1(&mut self) -> PROT_AREA_END1_W<PRAR_PRG1_SPEC, 16> {
        PROT_AREA_END1_W::new(self)
    }
    #[doc = "Bit 31 - Bank 1 PCROP protected erase enable option configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmep1(&mut self) -> DMEP1_W<PRAR_PRG1_SPEC, 31> {
        DMEP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH protection address for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prar_prg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prar_prg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRAR_PRG1_SPEC;
impl crate::RegisterSpec for PRAR_PRG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prar_prg1::R`](R) reader structure"]
impl crate::Readable for PRAR_PRG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prar_prg1::W`](W) writer structure"]
impl crate::Writable for PRAR_PRG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRAR_PRG1 to value 0"]
impl crate::Resettable for PRAR_PRG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
