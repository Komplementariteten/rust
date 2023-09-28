#[doc = "Register `MACMDIOAR` reader"]
pub type R = crate::R<MACMDIOAR_SPEC>;
#[doc = "Register `MACMDIOAR` writer"]
pub type W = crate::W<MACMDIOAR_SPEC>;
#[doc = "Field `MB` reader - MB"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - MB"]
pub type MB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C45E` reader - C45E"]
pub type C45E_R = crate::BitReader;
#[doc = "Field `C45E` writer - C45E"]
pub type C45E_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GOC` reader - GOC"]
pub type GOC_R = crate::FieldReader;
#[doc = "Field `GOC` writer - GOC"]
pub type GOC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SKAP` reader - SKAP"]
pub type SKAP_R = crate::BitReader;
#[doc = "Field `SKAP` writer - SKAP"]
pub type SKAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CR` reader - CR"]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - CR"]
pub type CR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `NTC` reader - NTC"]
pub type NTC_R = crate::FieldReader;
#[doc = "Field `NTC` writer - NTC"]
pub type NTC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RDA` reader - RDA"]
pub type RDA_R = crate::FieldReader;
#[doc = "Field `RDA` writer - RDA"]
pub type RDA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PA` reader - PA"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PA"]
pub type PA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `BTB` reader - BTB"]
pub type BTB_R = crate::BitReader;
#[doc = "Field `BTB` writer - BTB"]
pub type BTB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PSE` reader - PSE"]
pub type PSE_R = crate::BitReader;
#[doc = "Field `PSE` writer - PSE"]
pub type PSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - MB"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MB"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<MACMDIOAR_SPEC, 0> {
        MB_W::new(self)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    #[must_use]
    pub fn c45e(&mut self) -> C45E_W<MACMDIOAR_SPEC, 1> {
        C45E_W::new(self)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    #[must_use]
    pub fn goc(&mut self) -> GOC_W<MACMDIOAR_SPEC, 2> {
        GOC_W::new(self)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    #[must_use]
    pub fn skap(&mut self) -> SKAP_W<MACMDIOAR_SPEC, 4> {
        SKAP_W::new(self)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MACMDIOAR_SPEC, 8> {
        CR_W::new(self)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    #[must_use]
    pub fn ntc(&mut self) -> NTC_W<MACMDIOAR_SPEC, 12> {
        NTC_W::new(self)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    #[must_use]
    pub fn rda(&mut self) -> RDA_W<MACMDIOAR_SPEC, 16> {
        RDA_W::new(self)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<MACMDIOAR_SPEC, 21> {
        PA_W::new(self)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    #[must_use]
    pub fn btb(&mut self) -> BTB_W<MACMDIOAR_SPEC, 26> {
        BTB_W::new(self)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<MACMDIOAR_SPEC, 27> {
        PSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MDIO address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmdioar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmdioar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMDIOAR_SPEC;
impl crate::RegisterSpec for MACMDIOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdioar::R`](R) reader structure"]
impl crate::Readable for MACMDIOAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmdioar::W`](W) writer structure"]
impl crate::Writable for MACMDIOAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMDIOAR to value 0"]
impl crate::Resettable for MACMDIOAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
