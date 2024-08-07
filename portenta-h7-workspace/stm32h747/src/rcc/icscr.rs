#[doc = "Register `ICSCR` reader"]
pub type R = crate::R<ICSCR_SPEC>;
#[doc = "Register `ICSCR` writer"]
pub type W = crate::W<ICSCR_SPEC>;
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub type HSICAL_R = crate::FieldReader<u16>;
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub type HSITRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `CSICAL` reader - CSI clock calibration"]
pub type CSICAL_R = crate::FieldReader;
#[doc = "Field `CSITRIM` reader - CSI clock trimming"]
pub type CSITRIM_R = crate::FieldReader;
#[doc = "Field `CSITRIM` writer - CSI clock trimming"]
pub type CSITRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:25 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:30 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - HSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<ICSCR_SPEC, 12> {
        HSITRIM_W::new(self)
    }
    #[doc = "Bits 26:30 - CSI clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<ICSCR_SPEC, 26> {
        CSITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RCC Internal Clock Source Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICSCR_SPEC;
impl crate::RegisterSpec for ICSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icscr::R`](R) reader structure"]
impl crate::Readable for ICSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icscr::W`](W) writer structure"]
impl crate::Writable for ICSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSCR to value 0x4000_0000"]
impl crate::Resettable for ICSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
