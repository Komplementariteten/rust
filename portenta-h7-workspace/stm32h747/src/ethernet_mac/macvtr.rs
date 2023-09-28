#[doc = "Register `MACVTR` reader"]
pub type R = crate::R<MACVTR_SPEC>;
#[doc = "Register `MACVTR` writer"]
pub type W = crate::W<MACVTR_SPEC>;
#[doc = "Field `VL` reader - VL"]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VL"]
pub type VL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `ETV` reader - ETV"]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - ETV"]
pub type ETV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VTIM` reader - VTIM"]
pub type VTIM_R = crate::BitReader;
#[doc = "Field `VTIM` writer - VTIM"]
pub type VTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESVL` reader - ESVL"]
pub type ESVL_R = crate::BitReader;
#[doc = "Field `ESVL` writer - ESVL"]
pub type ESVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERSVLM` reader - ERSVLM"]
pub type ERSVLM_R = crate::BitReader;
#[doc = "Field `ERSVLM` writer - ERSVLM"]
pub type ERSVLM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DOVLTC` reader - DOVLTC"]
pub type DOVLTC_R = crate::BitReader;
#[doc = "Field `DOVLTC` writer - DOVLTC"]
pub type DOVLTC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EVLS` reader - EVLS"]
pub type EVLS_R = crate::FieldReader;
#[doc = "Field `EVLS` writer - EVLS"]
pub type EVLS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EVLRXS` reader - EVLRXS"]
pub type EVLRXS_R = crate::BitReader;
#[doc = "Field `EVLRXS` writer - EVLRXS"]
pub type EVLRXS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VTHM` reader - VTHM"]
pub type VTHM_R = crate::BitReader;
#[doc = "Field `VTHM` writer - VTHM"]
pub type VTHM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EDVLP` reader - EDVLP"]
pub type EDVLP_R = crate::BitReader;
#[doc = "Field `EDVLP` writer - EDVLP"]
pub type EDVLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERIVLT` reader - ERIVLT"]
pub type ERIVLT_R = crate::BitReader;
#[doc = "Field `ERIVLT` writer - ERIVLT"]
pub type ERIVLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIVLS` reader - EIVLS"]
pub type EIVLS_R = crate::FieldReader;
#[doc = "Field `EIVLS` writer - EIVLS"]
pub type EIVLS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EIVLRXS` reader - EIVLRXS"]
pub type EIVLRXS_R = crate::BitReader;
#[doc = "Field `EIVLRXS` writer - EIVLRXS"]
pub type EIVLRXS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<MACVTR_SPEC, 0> {
        VL_W::new(self)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<MACVTR_SPEC, 16> {
        ETV_W::new(self)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    #[must_use]
    pub fn vtim(&mut self) -> VTIM_W<MACVTR_SPEC, 17> {
        VTIM_W::new(self)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    #[must_use]
    pub fn esvl(&mut self) -> ESVL_W<MACVTR_SPEC, 18> {
        ESVL_W::new(self)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    #[must_use]
    pub fn ersvlm(&mut self) -> ERSVLM_W<MACVTR_SPEC, 19> {
        ERSVLM_W::new(self)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    #[must_use]
    pub fn dovltc(&mut self) -> DOVLTC_W<MACVTR_SPEC, 20> {
        DOVLTC_W::new(self)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    #[must_use]
    pub fn evls(&mut self) -> EVLS_W<MACVTR_SPEC, 21> {
        EVLS_W::new(self)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    #[must_use]
    pub fn evlrxs(&mut self) -> EVLRXS_W<MACVTR_SPEC, 24> {
        EVLRXS_W::new(self)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    #[must_use]
    pub fn vthm(&mut self) -> VTHM_W<MACVTR_SPEC, 25> {
        VTHM_W::new(self)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    #[must_use]
    pub fn edvlp(&mut self) -> EDVLP_W<MACVTR_SPEC, 26> {
        EDVLP_W::new(self)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    #[must_use]
    pub fn erivlt(&mut self) -> ERIVLT_W<MACVTR_SPEC, 27> {
        ERIVLT_W::new(self)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    #[must_use]
    pub fn eivls(&mut self) -> EIVLS_W<MACVTR_SPEC, 28> {
        EIVLS_W::new(self)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    #[must_use]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<MACVTR_SPEC, 31> {
        EIVLRXS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVTR_SPEC;
impl crate::RegisterSpec for MACVTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvtr::R`](R) reader structure"]
impl crate::Readable for MACVTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macvtr::W`](W) writer structure"]
impl crate::Writable for MACVTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACVTR to value 0"]
impl crate::Resettable for MACVTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
