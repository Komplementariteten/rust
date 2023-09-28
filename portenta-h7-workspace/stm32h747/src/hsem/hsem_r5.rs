#[doc = "Register `HSEM_R5` reader"]
pub type R = crate::R<HSEM_R5_SPEC>;
#[doc = "Register `HSEM_R5` writer"]
pub type W = crate::W<HSEM_R5_SPEC>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `PROCID` writer - Semaphore ProcessID"]
pub type PROCID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `COREID` reader - Semaphore COREID"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `COREID` writer - Semaphore COREID"]
pub type COREID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock indication"]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    #[must_use]
    pub fn procid(&mut self) -> PROCID_W<HSEM_R5_SPEC, 0> {
        PROCID_W::new(self)
    }
    #[doc = "Bits 8:11 - Semaphore COREID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<HSEM_R5_SPEC, 8> {
        COREID_W::new(self)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<HSEM_R5_SPEC, 31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_r5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsem_r5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_R5_SPEC;
impl crate::RegisterSpec for HSEM_R5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_r5::R`](R) reader structure"]
impl crate::Readable for HSEM_R5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hsem_r5::W`](W) writer structure"]
impl crate::Writable for HSEM_R5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSEM_R5 to value 0"]
impl crate::Resettable for HSEM_R5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
