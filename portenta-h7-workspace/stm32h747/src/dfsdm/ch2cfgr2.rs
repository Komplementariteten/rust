#[doc = "Register `CH2CFGR2` reader"]
pub type R = crate::R<CH2CFGR2_SPEC>;
#[doc = "Register `CH2CFGR2` writer"]
pub type W = crate::W<CH2CFGR2_SPEC>;
#[doc = "Field `DTRBS` reader - DTRBS"]
pub type DTRBS_R = crate::FieldReader;
#[doc = "Field `DTRBS` writer - DTRBS"]
pub type DTRBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `OFFSET` reader - OFFSET"]
pub type OFFSET_R = crate::FieldReader<u32>;
#[doc = "Field `OFFSET` writer - OFFSET"]
pub type OFFSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    #[must_use]
    pub fn dtrbs(&mut self) -> DTRBS_W<CH2CFGR2_SPEC, 3> {
        DTRBS_W::new(self)
    }
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<CH2CFGR2_SPEC, 8> {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CH2CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2CFGR2_SPEC;
impl crate::RegisterSpec for CH2CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cfgr2::R`](R) reader structure"]
impl crate::Readable for CH2CFGR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2cfgr2::W`](W) writer structure"]
impl crate::Writable for CH2CFGR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CFGR2 to value 0"]
impl crate::Resettable for CH2CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
