#[doc = "Register `MACPPSTTNR` reader"]
pub type R = crate::R<MACPPSTTNR_SPEC>;
#[doc = "Register `MACPPSTTNR` writer"]
pub type W = crate::W<MACPPSTTNR_SPEC>;
#[doc = "Field `TTSL0` reader - TTSL0"]
pub type TTSL0_R = crate::FieldReader<u32>;
#[doc = "Field `TTSL0` writer - TTSL0"]
pub type TTSL0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
#[doc = "Field `TRGTBUSY0` reader - TRGTBUSY0"]
pub type TRGTBUSY0_R = crate::BitReader;
#[doc = "Field `TRGTBUSY0` writer - TRGTBUSY0"]
pub type TRGTBUSY0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:30 - TTSL0"]
    #[inline(always)]
    pub fn ttsl0(&self) -> TTSL0_R {
        TTSL0_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TRGTBUSY0"]
    #[inline(always)]
    pub fn trgtbusy0(&self) -> TRGTBUSY0_R {
        TRGTBUSY0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TTSL0"]
    #[inline(always)]
    #[must_use]
    pub fn ttsl0(&mut self) -> TTSL0_W<MACPPSTTNR_SPEC, 0> {
        TTSL0_W::new(self)
    }
    #[doc = "Bit 31 - TRGTBUSY0"]
    #[inline(always)]
    #[must_use]
    pub fn trgtbusy0(&mut self) -> TRGTBUSY0_W<MACPPSTTNR_SPEC, 31> {
        TRGTBUSY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPS target time nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macppsttnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macppsttnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPPSTTNR_SPEC;
impl crate::RegisterSpec for MACPPSTTNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macppsttnr::R`](R) reader structure"]
impl crate::Readable for MACPPSTTNR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macppsttnr::W`](W) writer structure"]
impl crate::Writable for MACPPSTTNR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPPSTTNR to value 0"]
impl crate::Resettable for MACPPSTTNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
