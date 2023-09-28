#[doc = "Register `CCR1` reader"]
pub type R = crate::R<CCR1_SPEC>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<CCR1_SPEC>;
#[doc = "Field `CLR_EOP1` reader - Bank 1 EOP1 flag clear bit"]
pub type CLR_EOP1_R = crate::BitReader;
#[doc = "Field `CLR_EOP1` writer - Bank 1 EOP1 flag clear bit"]
pub type CLR_EOP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_WRPERR1` reader - Bank 1 WRPERR1 flag clear bit"]
pub type CLR_WRPERR1_R = crate::BitReader;
#[doc = "Field `CLR_WRPERR1` writer - Bank 1 WRPERR1 flag clear bit"]
pub type CLR_WRPERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_PGSERR1` reader - Bank 1 PGSERR1 flag clear bi"]
pub type CLR_PGSERR1_R = crate::BitReader;
#[doc = "Field `CLR_PGSERR1` writer - Bank 1 PGSERR1 flag clear bi"]
pub type CLR_PGSERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_STRBERR1` reader - Bank 1 STRBERR1 flag clear bit"]
pub type CLR_STRBERR1_R = crate::BitReader;
#[doc = "Field `CLR_STRBERR1` writer - Bank 1 STRBERR1 flag clear bit"]
pub type CLR_STRBERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_INCERR1` reader - Bank 1 INCERR1 flag clear bit"]
pub type CLR_INCERR1_R = crate::BitReader;
#[doc = "Field `CLR_INCERR1` writer - Bank 1 INCERR1 flag clear bit"]
pub type CLR_INCERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_OPERR1` reader - Bank 1 OPERR1 flag clear bit"]
pub type CLR_OPERR1_R = crate::BitReader;
#[doc = "Field `CLR_OPERR1` writer - Bank 1 OPERR1 flag clear bit"]
pub type CLR_OPERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_RDPERR1` reader - Bank 1 RDPERR1 flag clear bit"]
pub type CLR_RDPERR1_R = crate::BitReader;
#[doc = "Field `CLR_RDPERR1` writer - Bank 1 RDPERR1 flag clear bit"]
pub type CLR_RDPERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_RDSERR1` reader - Bank 1 RDSERR1 flag clear bit"]
pub type CLR_RDSERR1_R = crate::BitReader;
#[doc = "Field `CLR_RDSERR1` writer - Bank 1 RDSERR1 flag clear bit"]
pub type CLR_RDSERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_SNECCERR1` reader - Bank 1 SNECCERR1 flag clear bit"]
pub type CLR_SNECCERR1_R = crate::BitReader;
#[doc = "Field `CLR_SNECCERR1` writer - Bank 1 SNECCERR1 flag clear bit"]
pub type CLR_SNECCERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_DBECCERR1` reader - Bank 1 DBECCERR1 flag clear bit"]
pub type CLR_DBECCERR1_R = crate::BitReader;
#[doc = "Field `CLR_DBECCERR1` writer - Bank 1 DBECCERR1 flag clear bit"]
pub type CLR_DBECCERR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLR_CRCEND1` reader - Bank 1 CRCEND1 flag clear bit"]
pub type CLR_CRCEND1_R = crate::BitReader;
#[doc = "Field `CLR_CRCEND1` writer - Bank 1 CRCEND1 flag clear bit"]
pub type CLR_CRCEND1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop1(&self) -> CLR_EOP1_R {
        CLR_EOP1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr1(&self) -> CLR_WRPERR1_R {
        CLR_WRPERR1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr1(&self) -> CLR_PGSERR1_R {
        CLR_PGSERR1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr1(&self) -> CLR_STRBERR1_R {
        CLR_STRBERR1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr1(&self) -> CLR_INCERR1_R {
        CLR_INCERR1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr1(&self) -> CLR_OPERR1_R {
        CLR_OPERR1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr1(&self) -> CLR_RDPERR1_R {
        CLR_RDPERR1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr1(&self) -> CLR_RDSERR1_R {
        CLR_RDSERR1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr1(&self) -> CLR_SNECCERR1_R {
        CLR_SNECCERR1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr1(&self) -> CLR_DBECCERR1_R {
        CLR_DBECCERR1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend1(&self) -> CLR_CRCEND1_R {
        CLR_CRCEND1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_eop1(&mut self) -> CLR_EOP1_W<CCR1_SPEC, 16> {
        CLR_EOP1_W::new(self)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_wrperr1(&mut self) -> CLR_WRPERR1_W<CCR1_SPEC, 17> {
        CLR_WRPERR1_W::new(self)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bi"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pgserr1(&mut self) -> CLR_PGSERR1_W<CCR1_SPEC, 18> {
        CLR_PGSERR1_W::new(self)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_strberr1(&mut self) -> CLR_STRBERR1_W<CCR1_SPEC, 19> {
        CLR_STRBERR1_W::new(self)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_incerr1(&mut self) -> CLR_INCERR1_W<CCR1_SPEC, 21> {
        CLR_INCERR1_W::new(self)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_operr1(&mut self) -> CLR_OPERR1_W<CCR1_SPEC, 22> {
        CLR_OPERR1_W::new(self)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rdperr1(&mut self) -> CLR_RDPERR1_W<CCR1_SPEC, 23> {
        CLR_RDPERR1_W::new(self)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rdserr1(&mut self) -> CLR_RDSERR1_W<CCR1_SPEC, 24> {
        CLR_RDSERR1_W::new(self)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_sneccerr1(&mut self) -> CLR_SNECCERR1_W<CCR1_SPEC, 25> {
        CLR_SNECCERR1_W::new(self)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_dbeccerr1(&mut self) -> CLR_DBECCERR1_W<CCR1_SPEC, 26> {
        CLR_DBECCERR1_W::new(self)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_crcend1(&mut self) -> CLR_CRCEND1_W<CCR1_SPEC, 27> {
        CLR_CRCEND1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FLASH clear control register for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR1_SPEC;
impl crate::RegisterSpec for CCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for CCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for CCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for CCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
