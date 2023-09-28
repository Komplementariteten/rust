#[doc = "Register `WPSN_PRG2R` reader"]
pub type R = crate::R<WPSN_PRG2R_SPEC>;
#[doc = "Register `WPSN_PRG2R` writer"]
pub type W = crate::W<WPSN_PRG2R_SPEC>;
#[doc = "Field `WRPSn2` reader - Bank 2 sector write protection configuration byte"]
pub type WRPSN2_R = crate::FieldReader;
#[doc = "Field `WRPSn2` writer - Bank 2 sector write protection configuration byte"]
pub type WRPSN2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bank 2 sector write protection configuration byte"]
    #[inline(always)]
    pub fn wrpsn2(&self) -> WRPSN2_R {
        WRPSN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bank 2 sector write protection configuration byte"]
    #[inline(always)]
    #[must_use]
    pub fn wrpsn2(&mut self) -> WRPSN2_W<WPSN_PRG2R_SPEC, 0> {
        WRPSN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH write sector protection for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_prg2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpsn_prg2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSN_PRG2R_SPEC;
impl crate::RegisterSpec for WPSN_PRG2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_prg2r::R`](R) reader structure"]
impl crate::Readable for WPSN_PRG2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wpsn_prg2r::W`](W) writer structure"]
impl crate::Writable for WPSN_PRG2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPSN_PRG2R to value 0"]
impl crate::Resettable for WPSN_PRG2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
