#[doc = "Register `MACSTNUR` reader"]
pub type R = crate::R<MACSTNUR_SPEC>;
#[doc = "Register `MACSTNUR` writer"]
pub type W = crate::W<MACSTNUR_SPEC>;
#[doc = "Field `TSSS` reader - TSSS"]
pub type TSSS_R = crate::FieldReader<u32>;
#[doc = "Field `TSSS` writer - TSSS"]
pub type TSSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 31, O, u32>;
#[doc = "Field `ADDSUB` reader - ADDSUB"]
pub type ADDSUB_R = crate::BitReader;
#[doc = "Field `ADDSUB` writer - ADDSUB"]
pub type ADDSUB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    #[must_use]
    pub fn tsss(&mut self) -> TSSS_W<MACSTNUR_SPEC, 0> {
        TSSS_W::new(self)
    }
    #[doc = "Bit 31 - ADDSUB"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> ADDSUB_W<MACSTNUR_SPEC, 31> {
        ADDSUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System time nanoseconds update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macstnur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macstnur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSTNUR_SPEC;
impl crate::RegisterSpec for MACSTNUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstnur::R`](R) reader structure"]
impl crate::Readable for MACSTNUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macstnur::W`](W) writer structure"]
impl crate::Writable for MACSTNUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACSTNUR to value 0"]
impl crate::Resettable for MACSTNUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
