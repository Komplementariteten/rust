#[doc = "Register `CCR2` reader"]
pub type R = crate::R<CCR2_SPEC>;
#[doc = "Register `CCR2` writer"]
pub type W = crate::W<CCR2_SPEC>;
#[doc = "Field `CLR_EOP2` reader - Bank 1 EOP1 flag clear bit"]
pub type CLR_EOP2_R = crate::BitReader;
#[doc = "Field `CLR_EOP2` writer - Bank 1 EOP1 flag clear bit"]
pub type CLR_EOP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_WRPERR2` reader - Bank 2 WRPERR1 flag clear bit"]
pub type CLR_WRPERR2_R = crate::BitReader;
#[doc = "Field `CLR_WRPERR2` writer - Bank 2 WRPERR1 flag clear bit"]
pub type CLR_WRPERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_PGSERR2` reader - Bank 2 PGSERR1 flag clear bi"]
pub type CLR_PGSERR2_R = crate::BitReader;
#[doc = "Field `CLR_PGSERR2` writer - Bank 2 PGSERR1 flag clear bi"]
pub type CLR_PGSERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_STRBERR2` reader - Bank 2 STRBERR1 flag clear bit"]
pub type CLR_STRBERR2_R = crate::BitReader;
#[doc = "Field `CLR_STRBERR2` writer - Bank 2 STRBERR1 flag clear bit"]
pub type CLR_STRBERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_INCERR2` reader - Bank 2 INCERR1 flag clear bit"]
pub type CLR_INCERR2_R = crate::BitReader;
#[doc = "Field `CLR_INCERR2` writer - Bank 2 INCERR1 flag clear bit"]
pub type CLR_INCERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_OPERR2` reader - Bank 2 OPERR1 flag clear bit"]
pub type CLR_OPERR2_R = crate::BitReader;
#[doc = "Field `CLR_OPERR2` writer - Bank 2 OPERR1 flag clear bit"]
pub type CLR_OPERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_RDPERR2` reader - Bank 2 RDPERR1 flag clear bit"]
pub type CLR_RDPERR2_R = crate::BitReader;
#[doc = "Field `CLR_RDPERR2` writer - Bank 2 RDPERR1 flag clear bit"]
pub type CLR_RDPERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_RDSERR1` reader - Bank 1 RDSERR1 flag clear bit"]
pub type CLR_RDSERR1_R = crate::BitReader;
#[doc = "Field `CLR_RDSERR1` writer - Bank 1 RDSERR1 flag clear bit"]
pub type CLR_RDSERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_SNECCERR2` reader - Bank 2 SNECCERR1 flag clear bit"]
pub type CLR_SNECCERR2_R = crate::BitReader;
#[doc = "Field `CLR_SNECCERR2` writer - Bank 2 SNECCERR1 flag clear bit"]
pub type CLR_SNECCERR2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_DBECCERR1` reader - Bank 1 DBECCERR1 flag clear bit"]
pub type CLR_DBECCERR1_R = crate::BitReader;
#[doc = "Field `CLR_DBECCERR1` writer - Bank 1 DBECCERR1 flag clear bit"]
pub type CLR_DBECCERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_CRCEND2` reader - Bank 2 CRCEND1 flag clear bit"]
pub type CLR_CRCEND2_R = crate::BitReader;
#[doc = "Field `CLR_CRCEND2` writer - Bank 2 CRCEND1 flag clear bit"]
pub type CLR_CRCEND2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop2(&self) -> CLR_EOP2_R {
        CLR_EOP2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr2(&self) -> CLR_WRPERR2_R {
        CLR_WRPERR2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 2 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr2(&self) -> CLR_PGSERR2_R {
        CLR_PGSERR2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 2 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr2(&self) -> CLR_STRBERR2_R {
        CLR_STRBERR2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 2 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr2(&self) -> CLR_INCERR2_R {
        CLR_INCERR2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 2 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr2(&self) -> CLR_OPERR2_R {
        CLR_OPERR2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 2 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr2(&self) -> CLR_RDPERR2_R {
        CLR_RDPERR2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr1(&self) -> CLR_RDSERR1_R {
        CLR_RDSERR1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 2 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr2(&self) -> CLR_SNECCERR2_R {
        CLR_SNECCERR2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr1(&self) -> CLR_DBECCERR1_R {
        CLR_DBECCERR1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 2 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend2(&self) -> CLR_CRCEND2_R {
        CLR_CRCEND2_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_eop2(&mut self) -> CLR_EOP2_W<CCR2_SPEC, 16> {
        CLR_EOP2_W::new(self)
    }
    #[doc = "Bit 17 - Bank 2 WRPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_wrperr2(&mut self) -> CLR_WRPERR2_W<CCR2_SPEC, 17> {
        CLR_WRPERR2_W::new(self)
    }
    #[doc = "Bit 18 - Bank 2 PGSERR1 flag clear bi"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pgserr2(&mut self) -> CLR_PGSERR2_W<CCR2_SPEC, 18> {
        CLR_PGSERR2_W::new(self)
    }
    #[doc = "Bit 19 - Bank 2 STRBERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_strberr2(&mut self) -> CLR_STRBERR2_W<CCR2_SPEC, 19> {
        CLR_STRBERR2_W::new(self)
    }
    #[doc = "Bit 21 - Bank 2 INCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_incerr2(&mut self) -> CLR_INCERR2_W<CCR2_SPEC, 21> {
        CLR_INCERR2_W::new(self)
    }
    #[doc = "Bit 22 - Bank 2 OPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_operr2(&mut self) -> CLR_OPERR2_W<CCR2_SPEC, 22> {
        CLR_OPERR2_W::new(self)
    }
    #[doc = "Bit 23 - Bank 2 RDPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rdperr2(&mut self) -> CLR_RDPERR2_W<CCR2_SPEC, 23> {
        CLR_RDPERR2_W::new(self)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rdserr1(&mut self) -> CLR_RDSERR1_W<CCR2_SPEC, 24> {
        CLR_RDSERR1_W::new(self)
    }
    #[doc = "Bit 25 - Bank 2 SNECCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_sneccerr2(&mut self) -> CLR_SNECCERR2_W<CCR2_SPEC, 25> {
        CLR_SNECCERR2_W::new(self)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_dbeccerr1(&mut self) -> CLR_DBECCERR1_W<CCR2_SPEC, 26> {
        CLR_DBECCERR1_W::new(self)
    }
    #[doc = "Bit 27 - Bank 2 CRCEND1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_crcend2(&mut self) -> CLR_CRCEND2_W<CCR2_SPEC, 27> {
        CLR_CRCEND2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH clear control register for bank 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR2_SPEC;
impl crate::RegisterSpec for CCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2::R`](R) reader structure"]
impl crate::Readable for CCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr2::W`](W) writer structure"]
impl crate::Writable for CCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR2 to value 0"]
impl crate::Resettable for CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
