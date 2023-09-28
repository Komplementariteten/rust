#[doc = "Register `LIFCR` reader"]
pub type R = crate::R<LIFCR_SPEC>;
#[doc = "Register `LIFCR` writer"]
pub type W = crate::W<LIFCR_SPEC>;
#[doc = "Field `CFEIF0` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF0_R = crate::BitReader;
#[doc = "Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDMEIF0` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF0_R = crate::BitReader;
#[doc = "Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF0` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF0_R = crate::BitReader;
#[doc = "Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF0` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF0_R = crate::BitReader;
#[doc = "Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF0` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF0_R = crate::BitReader;
#[doc = "Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFEIF1` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF1_R = crate::BitReader;
#[doc = "Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDMEIF1` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF1_R = crate::BitReader;
#[doc = "Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF1` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF1_R = crate::BitReader;
#[doc = "Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF1` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF1_R = crate::BitReader;
#[doc = "Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF1` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF1_R = crate::BitReader;
#[doc = "Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFEIF2` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF2_R = crate::BitReader;
#[doc = "Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDMEIF2` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF2_R = crate::BitReader;
#[doc = "Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF2` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF2_R = crate::BitReader;
#[doc = "Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF2` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF2_R = crate::BitReader;
#[doc = "Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF2` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF2_R = crate::BitReader;
#[doc = "Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFEIF3` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF3_R = crate::BitReader;
#[doc = "Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CDMEIF3` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF3_R = crate::BitReader;
#[doc = "Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTEIF3` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF3_R = crate::BitReader;
#[doc = "Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHTIF3` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF3_R = crate::BitReader;
#[doc = "Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTCIF3` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF3_R = crate::BitReader;
#[doc = "Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&self) -> CFEIF0_R {
        CFEIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&self) -> CDMEIF0_R {
        CDMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&self) -> CTEIF0_R {
        CTEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&self) -> CHTIF0_R {
        CHTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&self) -> CTCIF0_R {
        CTCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&self) -> CFEIF1_R {
        CFEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&self) -> CDMEIF1_R {
        CDMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&self) -> CFEIF2_R {
        CFEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&self) -> CDMEIF2_R {
        CDMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&self) -> CFEIF3_R {
        CFEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&self) -> CDMEIF3_R {
        CDMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif0(&mut self) -> CFEIF0_W<LIFCR_SPEC, 0> {
        CFEIF0_W::new(self)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<LIFCR_SPEC, 2> {
        CDMEIF0_W::new(self)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif0(&mut self) -> CTEIF0_W<LIFCR_SPEC, 3> {
        CTEIF0_W::new(self)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif0(&mut self) -> CHTIF0_W<LIFCR_SPEC, 4> {
        CHTIF0_W::new(self)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif0(&mut self) -> CTCIF0_W<LIFCR_SPEC, 5> {
        CTCIF0_W::new(self)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif1(&mut self) -> CFEIF1_W<LIFCR_SPEC, 6> {
        CFEIF1_W::new(self)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<LIFCR_SPEC, 8> {
        CDMEIF1_W::new(self)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<LIFCR_SPEC, 9> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<LIFCR_SPEC, 10> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<LIFCR_SPEC, 11> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif2(&mut self) -> CFEIF2_W<LIFCR_SPEC, 16> {
        CFEIF2_W::new(self)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<LIFCR_SPEC, 18> {
        CDMEIF2_W::new(self)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<LIFCR_SPEC, 19> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<LIFCR_SPEC, 20> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<LIFCR_SPEC, 21> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif3(&mut self) -> CFEIF3_W<LIFCR_SPEC, 22> {
        CFEIF3_W::new(self)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<LIFCR_SPEC, 24> {
        CDMEIF3_W::new(self)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<LIFCR_SPEC, 25> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<LIFCR_SPEC, 26> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<LIFCR_SPEC, 27> {
        CTCIF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "low interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lifcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIFCR_SPEC;
impl crate::RegisterSpec for LIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lifcr::R`](R) reader structure"]
impl crate::Readable for LIFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lifcr::W`](W) writer structure"]
impl crate::Writable for LIFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LIFCR to value 0"]
impl crate::Resettable for LIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
