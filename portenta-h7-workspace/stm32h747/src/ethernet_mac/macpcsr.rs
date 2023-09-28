#[doc = "Register `MACPCSR` reader"]
pub type R = crate::R<MACPCSR_SPEC>;
#[doc = "Register `MACPCSR` writer"]
pub type W = crate::W<MACPCSR_SPEC>;
#[doc = "Field `PWRDWN` reader - PWRDWN"]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `PWRDWN` writer - PWRDWN"]
pub type PWRDWN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MGKPKTEN` reader - MGKPKTEN"]
pub type MGKPKTEN_R = crate::BitReader;
#[doc = "Field `MGKPKTEN` writer - MGKPKTEN"]
pub type MGKPKTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWKPKTEN` reader - RWKPKTEN"]
pub type RWKPKTEN_R = crate::BitReader;
#[doc = "Field `RWKPKTEN` writer - RWKPKTEN"]
pub type RWKPKTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MGKPRCVD` reader - MGKPRCVD"]
pub type MGKPRCVD_R = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - RWKPRCVD"]
pub type RWKPRCVD_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - GLBLUCAST"]
pub type GLBLUCAST_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` writer - GLBLUCAST"]
pub type GLBLUCAST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWKPFE` reader - RWKPFE"]
pub type RWKPFE_R = crate::BitReader;
#[doc = "Field `RWKPFE` writer - RWKPFE"]
pub type RWKPFE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWKPTR` reader - RWKPTR"]
pub type RWKPTR_R = crate::FieldReader;
#[doc = "Field `RWKPTR` writer - RWKPTR"]
pub type RWKPTR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `RWKFILTRST` reader - RWKFILTRST"]
pub type RWKFILTRST_R = crate::BitReader;
#[doc = "Field `RWKFILTRST` writer - RWKFILTRST"]
pub type RWKFILTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - MGKPRCVD"]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWKPRCVD"]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    pub fn rwkpfe(&self) -> RWKPFE_R {
        RWKPFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWRDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<MACPCSR_SPEC, 0> {
        PWRDWN_W::new(self)
    }
    #[doc = "Bit 1 - MGKPKTEN"]
    #[inline(always)]
    #[must_use]
    pub fn mgkpkten(&mut self) -> MGKPKTEN_W<MACPCSR_SPEC, 1> {
        MGKPKTEN_W::new(self)
    }
    #[doc = "Bit 2 - RWKPKTEN"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpkten(&mut self) -> RWKPKTEN_W<MACPCSR_SPEC, 2> {
        RWKPKTEN_W::new(self)
    }
    #[doc = "Bit 9 - GLBLUCAST"]
    #[inline(always)]
    #[must_use]
    pub fn glblucast(&mut self) -> GLBLUCAST_W<MACPCSR_SPEC, 9> {
        GLBLUCAST_W::new(self)
    }
    #[doc = "Bit 10 - RWKPFE"]
    #[inline(always)]
    #[must_use]
    pub fn rwkpfe(&mut self) -> RWKPFE_W<MACPCSR_SPEC, 10> {
        RWKPFE_W::new(self)
    }
    #[doc = "Bits 24:28 - RWKPTR"]
    #[inline(always)]
    #[must_use]
    pub fn rwkptr(&mut self) -> RWKPTR_W<MACPCSR_SPEC, 24> {
        RWKPTR_W::new(self)
    }
    #[doc = "Bit 31 - RWKFILTRST"]
    #[inline(always)]
    #[must_use]
    pub fn rwkfiltrst(&mut self) -> RWKFILTRST_W<MACPCSR_SPEC, 31> {
        RWKFILTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PMT control status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPCSR_SPEC;
impl crate::RegisterSpec for MACPCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpcsr::R`](R) reader structure"]
impl crate::Readable for MACPCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macpcsr::W`](W) writer structure"]
impl crate::Writable for MACPCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPCSR to value 0"]
impl crate::Resettable for MACPCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
