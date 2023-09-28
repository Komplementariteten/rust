#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<AHB2RSTR_SPEC>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<AHB2RSTR_SPEC>;
#[doc = "Field `CAMITFRST` reader - CAMITF block reset"]
pub type CAMITFRST_R = crate::BitReader;
#[doc = "Field `CAMITFRST` writer - CAMITF block reset"]
pub type CAMITFRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRYPTRST` reader - Cryptography block reset"]
pub type CRYPTRST_R = crate::BitReader;
#[doc = "Field `CRYPTRST` writer - Cryptography block reset"]
pub type CRYPTRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HASHRST` reader - Hash block reset"]
pub type HASHRST_R = crate::BitReader;
#[doc = "Field `HASHRST` writer - Hash block reset"]
pub type HASHRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RNGRST` reader - Random Number Generator block reset"]
pub type RNGRST_R = crate::BitReader;
#[doc = "Field `RNGRST` writer - Random Number Generator block reset"]
pub type RNGRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 Delay block reset"]
pub type SDMMC2RST_R = crate::BitReader;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 Delay block reset"]
pub type SDMMC2RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&self) -> CAMITFRST_R {
        CAMITFRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&self) -> CRYPTRST_R {
        CRYPTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    #[must_use]
    pub fn camitfrst(&mut self) -> CAMITFRST_W<AHB2RSTR_SPEC, 0> {
        CAMITFRST_W::new(self)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    #[must_use]
    pub fn cryptrst(&mut self) -> CRYPTRST_W<AHB2RSTR_SPEC, 4> {
        CRYPTRST_W::new(self)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<AHB2RSTR_SPEC, 5> {
        HASHRST_W::new(self)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<AHB2RSTR_SPEC, 6> {
        RNGRST_W::new(self)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<AHB2RSTR_SPEC, 9> {
        SDMMC2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC AHB2 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb2rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for AHB2RSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for AHB2RSTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for AHB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
