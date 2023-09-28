#[doc = "Register `SAI_PDMCR` reader"]
pub type R = crate::R<SAI_PDMCR_SPEC>;
#[doc = "Register `SAI_PDMCR` writer"]
pub type W = crate::W<SAI_PDMCR_SPEC>;
#[doc = "Field `PDMEN` reader - PDM enable"]
pub type PDMEN_R = crate::BitReader;
#[doc = "Field `PDMEN` writer - PDM enable"]
pub type PDMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MICNBR` reader - Number of microphones"]
pub type MICNBR_R = crate::FieldReader;
#[doc = "Field `MICNBR` writer - Number of microphones"]
pub type MICNBR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CKEN1` reader - Clock enable of bitstream clock number 1"]
pub type CKEN1_R = crate::BitReader;
#[doc = "Field `CKEN1` writer - Clock enable of bitstream clock number 1"]
pub type CKEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEN2` reader - Clock enable of bitstream clock number 2"]
pub type CKEN2_R = crate::BitReader;
#[doc = "Field `CKEN2` writer - Clock enable of bitstream clock number 2"]
pub type CKEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEN3` reader - Clock enable of bitstream clock number 3"]
pub type CKEN3_R = crate::BitReader;
#[doc = "Field `CKEN3` writer - Clock enable of bitstream clock number 3"]
pub type CKEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CKEN4` reader - Clock enable of bitstream clock number 4"]
pub type CKEN4_R = crate::BitReader;
#[doc = "Field `CKEN4` writer - Clock enable of bitstream clock number 4"]
pub type CKEN4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Number of microphones"]
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock enable of bitstream clock number 2"]
    #[inline(always)]
    pub fn cken2(&self) -> CKEN2_R {
        CKEN2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock enable of bitstream clock number 3"]
    #[inline(always)]
    pub fn cken3(&self) -> CKEN3_R {
        CKEN3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable of bitstream clock number 4"]
    #[inline(always)]
    pub fn cken4(&self) -> CKEN4_R {
        CKEN4_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdmen(&mut self) -> PDMEN_W<SAI_PDMCR_SPEC, 0> {
        PDMEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Number of microphones"]
    #[inline(always)]
    #[must_use]
    pub fn micnbr(&mut self) -> MICNBR_W<SAI_PDMCR_SPEC, 4> {
        MICNBR_W::new(self)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    #[must_use]
    pub fn cken1(&mut self) -> CKEN1_W<SAI_PDMCR_SPEC, 8> {
        CKEN1_W::new(self)
    }
    #[doc = "Bit 9 - Clock enable of bitstream clock number 2"]
    #[inline(always)]
    #[must_use]
    pub fn cken2(&mut self) -> CKEN2_W<SAI_PDMCR_SPEC, 9> {
        CKEN2_W::new(self)
    }
    #[doc = "Bit 10 - Clock enable of bitstream clock number 3"]
    #[inline(always)]
    #[must_use]
    pub fn cken3(&mut self) -> CKEN3_W<SAI_PDMCR_SPEC, 10> {
        CKEN3_W::new(self)
    }
    #[doc = "Bit 11 - Clock enable of bitstream clock number 4"]
    #[inline(always)]
    #[must_use]
    pub fn cken4(&mut self) -> CKEN4_W<SAI_PDMCR_SPEC, 11> {
        CKEN4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_PDMCR_SPEC;
impl crate::RegisterSpec for SAI_PDMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_pdmcr::R`](R) reader structure"]
impl crate::Readable for SAI_PDMCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sai_pdmcr::W`](W) writer structure"]
impl crate::Writable for SAI_PDMCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAI_PDMCR to value 0"]
impl crate::Resettable for SAI_PDMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
