#[doc = "Register `DSI_FIR1` writer"]
pub type W = crate::W<DSI_FIR1_SPEC>;
#[doc = "Field `FTOHSTX` writer - FTOHSTX"]
pub type FTOHSTX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FTOLPRX` writer - FTOLPRX"]
pub type FTOLPRX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FECCSE` writer - FECCSE"]
pub type FECCSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FECCME` writer - FECCME"]
pub type FECCME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCRCE` writer - FCRCE"]
pub type FCRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FPSE` writer - FPSE"]
pub type FPSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEOTPE` writer - FEOTPE"]
pub type FEOTPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLPWRE` writer - FLPWRE"]
pub type FLPWRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FGCWRE` writer - FGCWRE"]
pub type FGCWRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FGPWRE` writer - FGPWRE"]
pub type FGPWRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FGPTXE` writer - FGPTXE"]
pub type FGPTXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FGPRDE` writer - FGPRDE"]
pub type FGPRDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FGPRXE` writer - FGPRXE"]
pub type FGPRXE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - FTOHSTX"]
    #[inline(always)]
    #[must_use]
    pub fn ftohstx(&mut self) -> FTOHSTX_W<DSI_FIR1_SPEC, 0> {
        FTOHSTX_W::new(self)
    }
    #[doc = "Bit 1 - FTOLPRX"]
    #[inline(always)]
    #[must_use]
    pub fn ftolprx(&mut self) -> FTOLPRX_W<DSI_FIR1_SPEC, 1> {
        FTOLPRX_W::new(self)
    }
    #[doc = "Bit 2 - FECCSE"]
    #[inline(always)]
    #[must_use]
    pub fn feccse(&mut self) -> FECCSE_W<DSI_FIR1_SPEC, 2> {
        FECCSE_W::new(self)
    }
    #[doc = "Bit 3 - FECCME"]
    #[inline(always)]
    #[must_use]
    pub fn feccme(&mut self) -> FECCME_W<DSI_FIR1_SPEC, 3> {
        FECCME_W::new(self)
    }
    #[doc = "Bit 4 - FCRCE"]
    #[inline(always)]
    #[must_use]
    pub fn fcrce(&mut self) -> FCRCE_W<DSI_FIR1_SPEC, 4> {
        FCRCE_W::new(self)
    }
    #[doc = "Bit 5 - FPSE"]
    #[inline(always)]
    #[must_use]
    pub fn fpse(&mut self) -> FPSE_W<DSI_FIR1_SPEC, 5> {
        FPSE_W::new(self)
    }
    #[doc = "Bit 6 - FEOTPE"]
    #[inline(always)]
    #[must_use]
    pub fn feotpe(&mut self) -> FEOTPE_W<DSI_FIR1_SPEC, 6> {
        FEOTPE_W::new(self)
    }
    #[doc = "Bit 7 - FLPWRE"]
    #[inline(always)]
    #[must_use]
    pub fn flpwre(&mut self) -> FLPWRE_W<DSI_FIR1_SPEC, 7> {
        FLPWRE_W::new(self)
    }
    #[doc = "Bit 8 - FGCWRE"]
    #[inline(always)]
    #[must_use]
    pub fn fgcwre(&mut self) -> FGCWRE_W<DSI_FIR1_SPEC, 8> {
        FGCWRE_W::new(self)
    }
    #[doc = "Bit 9 - FGPWRE"]
    #[inline(always)]
    #[must_use]
    pub fn fgpwre(&mut self) -> FGPWRE_W<DSI_FIR1_SPEC, 9> {
        FGPWRE_W::new(self)
    }
    #[doc = "Bit 10 - FGPTXE"]
    #[inline(always)]
    #[must_use]
    pub fn fgptxe(&mut self) -> FGPTXE_W<DSI_FIR1_SPEC, 10> {
        FGPTXE_W::new(self)
    }
    #[doc = "Bit 11 - FGPRDE"]
    #[inline(always)]
    #[must_use]
    pub fn fgprde(&mut self) -> FGPRDE_W<DSI_FIR1_SPEC, 11> {
        FGPRDE_W::new(self)
    }
    #[doc = "Bit 12 - FGPRXE"]
    #[inline(always)]
    #[must_use]
    pub fn fgprxe(&mut self) -> FGPRXE_W<DSI_FIR1_SPEC, 12> {
        FGPRXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host force interrupt register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_FIR1_SPEC;
impl crate::RegisterSpec for DSI_FIR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_fir1::W`](W) writer structure"]
impl crate::Writable for DSI_FIR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_FIR1 to value 0"]
impl crate::Resettable for DSI_FIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
