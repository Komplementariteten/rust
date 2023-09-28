#[doc = "Register `DSI_PUCR` reader"]
pub type R = crate::R<DSI_PUCR_SPEC>;
#[doc = "Register `DSI_PUCR` writer"]
pub type W = crate::W<DSI_PUCR_SPEC>;
#[doc = "Field `URCL` reader - URCL"]
pub type URCL_R = crate::BitReader;
#[doc = "Field `URCL` writer - URCL"]
pub type URCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UECL` reader - UECL"]
pub type UECL_R = crate::BitReader;
#[doc = "Field `UECL` writer - UECL"]
pub type UECL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `URDL` reader - URDL"]
pub type URDL_R = crate::BitReader;
#[doc = "Field `URDL` writer - URDL"]
pub type URDL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UEDL` reader - UEDL"]
pub type UEDL_R = crate::BitReader;
#[doc = "Field `UEDL` writer - UEDL"]
pub type UEDL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - URCL"]
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UECL"]
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URDL"]
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UEDL"]
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - URCL"]
    #[inline(always)]
    #[must_use]
    pub fn urcl(&mut self) -> URCL_W<DSI_PUCR_SPEC, 0> {
        URCL_W::new(self)
    }
    #[doc = "Bit 1 - UECL"]
    #[inline(always)]
    #[must_use]
    pub fn uecl(&mut self) -> UECL_W<DSI_PUCR_SPEC, 1> {
        UECL_W::new(self)
    }
    #[doc = "Bit 2 - URDL"]
    #[inline(always)]
    #[must_use]
    pub fn urdl(&mut self) -> URDL_W<DSI_PUCR_SPEC, 2> {
        URDL_W::new(self)
    }
    #[doc = "Bit 3 - UEDL"]
    #[inline(always)]
    #[must_use]
    pub fn uedl(&mut self) -> UEDL_W<DSI_PUCR_SPEC, 3> {
        UEDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host PHY ULPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_pucr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_pucr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_PUCR_SPEC;
impl crate::RegisterSpec for DSI_PUCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pucr::R`](R) reader structure"]
impl crate::Readable for DSI_PUCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_pucr::W`](W) writer structure"]
impl crate::Writable for DSI_PUCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_PUCR to value 0"]
impl crate::Resettable for DSI_PUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
