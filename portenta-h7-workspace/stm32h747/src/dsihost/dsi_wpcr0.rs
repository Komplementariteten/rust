#[doc = "Register `DSI_WPCR0` reader"]
pub type R = crate::R<DSI_WPCR0_SPEC>;
#[doc = "Register `DSI_WPCR0` writer"]
pub type W = crate::W<DSI_WPCR0_SPEC>;
#[doc = "Field `UIX4` reader - UIX4"]
pub type UIX4_R = crate::FieldReader;
#[doc = "Field `UIX4` writer - UIX4"]
pub type UIX4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SWCL` reader - SWCL"]
pub type SWCL_R = crate::BitReader;
#[doc = "Field `SWCL` writer - SWCL"]
pub type SWCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWDL0` reader - SWDL0"]
pub type SWDL0_R = crate::BitReader;
#[doc = "Field `SWDL0` writer - SWDL0"]
pub type SWDL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWDL1` reader - SWDL1"]
pub type SWDL1_R = crate::BitReader;
#[doc = "Field `SWDL1` writer - SWDL1"]
pub type SWDL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSICL` reader - HSICL"]
pub type HSICL_R = crate::BitReader;
#[doc = "Field `HSICL` writer - HSICL"]
pub type HSICL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIDL0` reader - HSIDL0"]
pub type HSIDL0_R = crate::BitReader;
#[doc = "Field `HSIDL0` writer - HSIDL0"]
pub type HSIDL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSIDL1` reader - HSIDL1"]
pub type HSIDL1_R = crate::BitReader;
#[doc = "Field `HSIDL1` writer - HSIDL1"]
pub type HSIDL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FTXSMCL` reader - FTXSMCL"]
pub type FTXSMCL_R = crate::BitReader;
#[doc = "Field `FTXSMCL` writer - FTXSMCL"]
pub type FTXSMCL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FTXSMDL` reader - FTXSMDL"]
pub type FTXSMDL_R = crate::BitReader;
#[doc = "Field `FTXSMDL` writer - FTXSMDL"]
pub type FTXSMDL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDOFFDL` reader - CDOFFDL"]
pub type CDOFFDL_R = crate::BitReader;
#[doc = "Field `CDOFFDL` writer - CDOFFDL"]
pub type CDOFFDL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TDDL` reader - TDDL"]
pub type TDDL_R = crate::BitReader;
#[doc = "Field `TDDL` writer - TDDL"]
pub type TDDL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PDEN` reader - Pull-down enable"]
pub type PDEN_R = crate::BitReader;
#[doc = "Field `PDEN` writer - Pull-down enable"]
pub type PDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCLKPREPEN` reader - Custom time for tCLK-PREPARE enable"]
pub type TCLKPREPEN_R = crate::BitReader;
#[doc = "Field `TCLKPREPEN` writer - Custom time for tCLK-PREPARE enable"]
pub type TCLKPREPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCLKZEROEN` reader - Custom time for tCLK-ZERO enable"]
pub type TCLKZEROEN_R = crate::BitReader;
#[doc = "Field `TCLKZEROEN` writer - Custom time for tCLK-ZERO enable"]
pub type TCLKZEROEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THSPREPEN` reader - Custom time for tHS-PREPARE enable"]
pub type THSPREPEN_R = crate::BitReader;
#[doc = "Field `THSPREPEN` writer - Custom time for tHS-PREPARE enable"]
pub type THSPREPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THSTRAILEN` reader - Custom time for tHS-TRAIL enable"]
pub type THSTRAILEN_R = crate::BitReader;
#[doc = "Field `THSTRAILEN` writer - Custom time for tHS-TRAIL enable"]
pub type THSTRAILEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THSZEROEN` reader - Custom time for tHS-ZERO enable"]
pub type THSZEROEN_R = crate::BitReader;
#[doc = "Field `THSZEROEN` writer - Custom time for tHS-ZERO enable"]
pub type THSZEROEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TLPXDEN` reader - Custom time for tLPX for data lanes enable"]
pub type TLPXDEN_R = crate::BitReader;
#[doc = "Field `TLPXDEN` writer - Custom time for tLPX for data lanes enable"]
pub type TLPXDEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `THSEXITEN` reader - Custom time for tHS-EXIT enable"]
pub type THSEXITEN_R = crate::BitReader;
#[doc = "Field `THSEXITEN` writer - Custom time for tHS-EXIT enable"]
pub type THSEXITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TLPXCEN` reader - Custom time for tLPX for clock lane enable"]
pub type TLPXCEN_R = crate::BitReader;
#[doc = "Field `TLPXCEN` writer - Custom time for tLPX for clock lane enable"]
pub type TLPXCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCLKPOSTEN` reader - Custom time for tCLK-POST enable"]
pub type TCLKPOSTEN_R = crate::BitReader;
#[doc = "Field `TCLKPOSTEN` writer - Custom time for tCLK-POST enable"]
pub type TCLKPOSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Custom time for tCLK-PREPARE enable"]
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Custom time for tCLK-ZERO enable"]
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Custom time for tHS-PREPARE enable"]
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Custom time for tHS-TRAIL enable"]
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Custom time for tHS-ZERO enable"]
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Custom time for tHS-EXIT enable"]
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Custom time for tCLK-POST enable"]
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    #[must_use]
    pub fn uix4(&mut self) -> UIX4_W<DSI_WPCR0_SPEC, 0> {
        UIX4_W::new(self)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    #[must_use]
    pub fn swcl(&mut self) -> SWCL_W<DSI_WPCR0_SPEC, 6> {
        SWCL_W::new(self)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    #[must_use]
    pub fn swdl0(&mut self) -> SWDL0_W<DSI_WPCR0_SPEC, 7> {
        SWDL0_W::new(self)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    #[must_use]
    pub fn swdl1(&mut self) -> SWDL1_W<DSI_WPCR0_SPEC, 8> {
        SWDL1_W::new(self)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    #[must_use]
    pub fn hsicl(&mut self) -> HSICL_W<DSI_WPCR0_SPEC, 9> {
        HSICL_W::new(self)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl0(&mut self) -> HSIDL0_W<DSI_WPCR0_SPEC, 10> {
        HSIDL0_W::new(self)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    #[must_use]
    pub fn hsidl1(&mut self) -> HSIDL1_W<DSI_WPCR0_SPEC, 11> {
        HSIDL1_W::new(self)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<DSI_WPCR0_SPEC, 12> {
        FTXSMCL_W::new(self)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    #[must_use]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<DSI_WPCR0_SPEC, 13> {
        FTXSMDL_W::new(self)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    #[must_use]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<DSI_WPCR0_SPEC, 14> {
        CDOFFDL_W::new(self)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    #[must_use]
    pub fn tddl(&mut self) -> TDDL_W<DSI_WPCR0_SPEC, 16> {
        TDDL_W::new(self)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn pden(&mut self) -> PDEN_W<DSI_WPCR0_SPEC, 18> {
        PDEN_W::new(self)
    }
    #[doc = "Bit 19 - Custom time for tCLK-PREPARE enable"]
    #[inline(always)]
    #[must_use]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<DSI_WPCR0_SPEC, 19> {
        TCLKPREPEN_W::new(self)
    }
    #[doc = "Bit 20 - Custom time for tCLK-ZERO enable"]
    #[inline(always)]
    #[must_use]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<DSI_WPCR0_SPEC, 20> {
        TCLKZEROEN_W::new(self)
    }
    #[doc = "Bit 21 - Custom time for tHS-PREPARE enable"]
    #[inline(always)]
    #[must_use]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<DSI_WPCR0_SPEC, 21> {
        THSPREPEN_W::new(self)
    }
    #[doc = "Bit 22 - Custom time for tHS-TRAIL enable"]
    #[inline(always)]
    #[must_use]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<DSI_WPCR0_SPEC, 22> {
        THSTRAILEN_W::new(self)
    }
    #[doc = "Bit 23 - Custom time for tHS-ZERO enable"]
    #[inline(always)]
    #[must_use]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<DSI_WPCR0_SPEC, 23> {
        THSZEROEN_W::new(self)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<DSI_WPCR0_SPEC, 24> {
        TLPXDEN_W::new(self)
    }
    #[doc = "Bit 25 - Custom time for tHS-EXIT enable"]
    #[inline(always)]
    #[must_use]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<DSI_WPCR0_SPEC, 25> {
        THSEXITEN_W::new(self)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    #[must_use]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<DSI_WPCR0_SPEC, 26> {
        TLPXCEN_W::new(self)
    }
    #[doc = "Bit 27 - Custom time for tCLK-POST enable"]
    #[inline(always)]
    #[must_use]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<DSI_WPCR0_SPEC, 27> {
        TCLKPOSTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_wpcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_wpcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_WPCR0_SPEC;
impl crate::RegisterSpec for DSI_WPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr0::R`](R) reader structure"]
impl crate::Readable for DSI_WPCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr0::W`](W) writer structure"]
impl crate::Writable for DSI_WPCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_WPCR0 to value 0"]
impl crate::Resettable for DSI_WPCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
