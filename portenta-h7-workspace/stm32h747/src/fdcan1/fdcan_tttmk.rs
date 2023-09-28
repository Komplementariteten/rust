#[doc = "Register `FDCAN_TTTMK` reader"]
pub type R = crate::R<FDCAN_TTTMK_SPEC>;
#[doc = "Register `FDCAN_TTTMK` writer"]
pub type W = crate::W<FDCAN_TTTMK_SPEC>;
#[doc = "Field `TM` reader - Time Mark"]
pub type TM_R = crate::FieldReader<u16>;
#[doc = "Field `TM` writer - Time Mark"]
pub type TM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TICC` reader - Time Mark Cycle Code"]
pub type TICC_R = crate::FieldReader;
#[doc = "Field `TICC` writer - Time Mark Cycle Code"]
pub type TICC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `LCKM` reader - TT Time Mark Register Locked"]
pub type LCKM_R = crate::BitReader;
#[doc = "Field `LCKM` writer - TT Time Mark Register Locked"]
pub type LCKM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Mark"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<FDCAN_TTTMK_SPEC, 0> {
        TM_W::new(self)
    }
    #[doc = "Bits 16:22 - Time Mark Cycle Code"]
    #[inline(always)]
    #[must_use]
    pub fn ticc(&mut self) -> TICC_W<FDCAN_TTTMK_SPEC, 16> {
        TICC_W::new(self)
    }
    #[doc = "Bit 31 - TT Time Mark Register Locked"]
    #[inline(always)]
    #[must_use]
    pub fn lckm(&mut self) -> LCKM_W<FDCAN_TTTMK_SPEC, 31> {
        LCKM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FDCAN TT Time Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_tttmk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_tttmk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTTMK_SPEC;
impl crate::RegisterSpec for FDCAN_TTTMK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tttmk::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTTMK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tttmk::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTTMK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTMK to value 0"]
impl crate::Resettable for FDCAN_TTTMK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
