#[doc = "Register `MACVIR` reader"]
pub type R = crate::R<MACVIR_SPEC>;
#[doc = "Register `MACVIR` writer"]
pub type W = crate::W<MACVIR_SPEC>;
#[doc = "Field `VLT` reader - VLT"]
pub type VLT_R = crate::FieldReader<u16>;
#[doc = "Field `VLT` writer - VLT"]
pub type VLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `VLC` reader - VLC"]
pub type VLC_R = crate::FieldReader;
#[doc = "Field `VLC` writer - VLC"]
pub type VLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VLP` reader - VLP"]
pub type VLP_R = crate::BitReader;
#[doc = "Field `VLP` writer - VLP"]
pub type VLP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSVL` reader - CSVL"]
pub type CSVL_R = crate::BitReader;
#[doc = "Field `CSVL` writer - CSVL"]
pub type CSVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VLTI` reader - VLTI"]
pub type VLTI_R = crate::BitReader;
#[doc = "Field `VLTI` writer - VLTI"]
pub type VLTI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<MACVIR_SPEC, 0> {
        VLT_W::new(self)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<MACVIR_SPEC, 16> {
        VLC_W::new(self)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<MACVIR_SPEC, 18> {
        VLP_W::new(self)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<MACVIR_SPEC, 19> {
        CSVL_W::new(self)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<MACVIR_SPEC, 20> {
        VLTI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "VLAN inclusion register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVIR_SPEC;
impl crate::RegisterSpec for MACVIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvir::R`](R) reader structure"]
impl crate::Readable for MACVIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macvir::W`](W) writer structure"]
impl crate::Writable for MACVIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACVIR to value 0"]
impl crate::Resettable for MACVIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
