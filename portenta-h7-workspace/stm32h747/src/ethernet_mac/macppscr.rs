#[doc = "Register `MACPPSCR` reader"]
pub type R = crate::R<MACPPSCR_SPEC>;
#[doc = "Register `MACPPSCR` writer"]
pub type W = crate::W<MACPPSCR_SPEC>;
#[doc = "Field `PPSCTRL` reader - PPSCTRL"]
pub type PPSCTRL_R = crate::FieldReader;
#[doc = "Field `PPSCTRL` writer - PPSCTRL"]
pub type PPSCTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PPSEN0` reader - PPSEN0"]
pub type PPSEN0_R = crate::BitReader;
#[doc = "Field `PPSEN0` writer - PPSEN0"]
pub type PPSEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGTMODSEL0` reader - TRGTMODSEL0"]
pub type TRGTMODSEL0_R = crate::FieldReader;
#[doc = "Field `TRGTMODSEL0` writer - TRGTMODSEL0"]
pub type TRGTMODSEL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PPSCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<MACPPSCR_SPEC, 0> {
        PPSCTRL_W::new(self)
    }
    #[doc = "Bit 4 - PPSEN0"]
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<MACPPSCR_SPEC, 4> {
        PPSEN0_W::new(self)
    }
    #[doc = "Bits 5:6 - TRGTMODSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<MACPPSCR_SPEC, 5> {
        TRGTMODSEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSCR_SPEC;
impl crate::RegisterSpec for MACPPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppscr::R`](R) reader structure"]
impl crate::Readable for MACPPSCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macppscr::W`](W) writer structure"]
impl crate::Writable for MACPPSCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPPSCR to value 0"]
impl crate::Resettable for MACPPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
