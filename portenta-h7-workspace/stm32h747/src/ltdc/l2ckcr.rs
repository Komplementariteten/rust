#[doc = "Register `L2CKCR` reader"]
pub type R = crate::R<L2CKCR_SPEC>;
#[doc = "Register `L2CKCR` writer"]
pub type W = crate::W<L2CKCR_SPEC>;
#[doc = "Field `CKBLUE` reader - Color Key Blue value"]
pub type CKBLUE_R = crate::FieldReader;
#[doc = "Field `CKBLUE` writer - Color Key Blue value"]
pub type CKBLUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CKGREEN` reader - Color Key Green value"]
pub type CKGREEN_R = crate::FieldReader;
#[doc = "Field `CKGREEN` writer - Color Key Green value"]
pub type CKGREEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CKRED` reader - Color Key Red value"]
pub type CKRED_R = crate::FieldReader;
#[doc = "Field `CKRED` writer - Color Key Red value"]
pub type CKRED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    #[must_use]
    pub fn ckblue(&mut self) -> CKBLUE_W<L2CKCR_SPEC, 0> {
        CKBLUE_W::new(self)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    #[must_use]
    pub fn ckgreen(&mut self) -> CKGREEN_W<L2CKCR_SPEC, 8> {
        CKGREEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    #[must_use]
    pub fn ckred(&mut self) -> CKRED_W<L2CKCR_SPEC, 16> {
        CKRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layerx Color Keying Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2ckcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2ckcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2CKCR_SPEC;
impl crate::RegisterSpec for L2CKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2ckcr::R`](R) reader structure"]
impl crate::Readable for L2CKCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2ckcr::W`](W) writer structure"]
impl crate::Writable for L2CKCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2CKCR to value 0"]
impl crate::Resettable for L2CKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
