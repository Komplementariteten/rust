#[doc = "Register `DSI_IER0` reader"]
pub type R = crate::R<DSI_IER0_SPEC>;
#[doc = "Register `DSI_IER0` writer"]
pub type W = crate::W<DSI_IER0_SPEC>;
#[doc = "Field `AE0IE` reader - AE0IE"]
pub type AE0IE_R = crate::BitReader;
#[doc = "Field `AE0IE` writer - AE0IE"]
pub type AE0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE1IE` reader - AE1IE"]
pub type AE1IE_R = crate::BitReader;
#[doc = "Field `AE1IE` writer - AE1IE"]
pub type AE1IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE2IE` reader - AE2IE"]
pub type AE2IE_R = crate::BitReader;
#[doc = "Field `AE2IE` writer - AE2IE"]
pub type AE2IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE3IE` reader - AE3IE"]
pub type AE3IE_R = crate::BitReader;
#[doc = "Field `AE3IE` writer - AE3IE"]
pub type AE3IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE4IE` reader - AE4IE"]
pub type AE4IE_R = crate::BitReader;
#[doc = "Field `AE4IE` writer - AE4IE"]
pub type AE4IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE5IE` reader - AE5IE"]
pub type AE5IE_R = crate::BitReader;
#[doc = "Field `AE5IE` writer - AE5IE"]
pub type AE5IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE6IE` reader - AE6IE"]
pub type AE6IE_R = crate::BitReader;
#[doc = "Field `AE6IE` writer - AE6IE"]
pub type AE6IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE7IE` reader - AE7IE"]
pub type AE7IE_R = crate::BitReader;
#[doc = "Field `AE7IE` writer - AE7IE"]
pub type AE7IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE8IE` reader - AE8IE"]
pub type AE8IE_R = crate::BitReader;
#[doc = "Field `AE8IE` writer - AE8IE"]
pub type AE8IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE9IE` reader - AE9IE"]
pub type AE9IE_R = crate::BitReader;
#[doc = "Field `AE9IE` writer - AE9IE"]
pub type AE9IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE10IE` reader - AE10IE"]
pub type AE10IE_R = crate::BitReader;
#[doc = "Field `AE10IE` writer - AE10IE"]
pub type AE10IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE11IE` reader - AE11IE"]
pub type AE11IE_R = crate::BitReader;
#[doc = "Field `AE11IE` writer - AE11IE"]
pub type AE11IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE12IE` reader - AE12IE"]
pub type AE12IE_R = crate::BitReader;
#[doc = "Field `AE12IE` writer - AE12IE"]
pub type AE12IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE13IE` reader - AE13IE"]
pub type AE13IE_R = crate::BitReader;
#[doc = "Field `AE13IE` writer - AE13IE"]
pub type AE13IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE14IE` reader - AE14IE"]
pub type AE14IE_R = crate::BitReader;
#[doc = "Field `AE14IE` writer - AE14IE"]
pub type AE14IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AE15IE` reader - AE15IE"]
pub type AE15IE_R = crate::BitReader;
#[doc = "Field `AE15IE` writer - AE15IE"]
pub type AE15IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE0IE` reader - PE0IE"]
pub type PE0IE_R = crate::BitReader;
#[doc = "Field `PE0IE` writer - PE0IE"]
pub type PE0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE1IE` reader - PE1IE"]
pub type PE1IE_R = crate::BitReader;
#[doc = "Field `PE1IE` writer - PE1IE"]
pub type PE1IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE2IE` reader - PE2IE"]
pub type PE2IE_R = crate::BitReader;
#[doc = "Field `PE2IE` writer - PE2IE"]
pub type PE2IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE3IE` reader - PE3IE"]
pub type PE3IE_R = crate::BitReader;
#[doc = "Field `PE3IE` writer - PE3IE"]
pub type PE3IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PE4IE` reader - PE4IE"]
pub type PE4IE_R = crate::BitReader;
#[doc = "Field `PE4IE` writer - PE4IE"]
pub type PE4IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - AE0IE"]
    #[inline(always)]
    pub fn ae0ie(&self) -> AE0IE_R {
        AE0IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AE1IE"]
    #[inline(always)]
    pub fn ae1ie(&self) -> AE1IE_R {
        AE1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AE2IE"]
    #[inline(always)]
    pub fn ae2ie(&self) -> AE2IE_R {
        AE2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AE3IE"]
    #[inline(always)]
    pub fn ae3ie(&self) -> AE3IE_R {
        AE3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AE4IE"]
    #[inline(always)]
    pub fn ae4ie(&self) -> AE4IE_R {
        AE4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AE5IE"]
    #[inline(always)]
    pub fn ae5ie(&self) -> AE5IE_R {
        AE5IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AE6IE"]
    #[inline(always)]
    pub fn ae6ie(&self) -> AE6IE_R {
        AE6IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AE7IE"]
    #[inline(always)]
    pub fn ae7ie(&self) -> AE7IE_R {
        AE7IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AE8IE"]
    #[inline(always)]
    pub fn ae8ie(&self) -> AE8IE_R {
        AE8IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AE9IE"]
    #[inline(always)]
    pub fn ae9ie(&self) -> AE9IE_R {
        AE9IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AE10IE"]
    #[inline(always)]
    pub fn ae10ie(&self) -> AE10IE_R {
        AE10IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AE11IE"]
    #[inline(always)]
    pub fn ae11ie(&self) -> AE11IE_R {
        AE11IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AE12IE"]
    #[inline(always)]
    pub fn ae12ie(&self) -> AE12IE_R {
        AE12IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AE13IE"]
    #[inline(always)]
    pub fn ae13ie(&self) -> AE13IE_R {
        AE13IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AE14IE"]
    #[inline(always)]
    pub fn ae14ie(&self) -> AE14IE_R {
        AE14IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AE15IE"]
    #[inline(always)]
    pub fn ae15ie(&self) -> AE15IE_R {
        AE15IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PE0IE"]
    #[inline(always)]
    pub fn pe0ie(&self) -> PE0IE_R {
        PE0IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PE1IE"]
    #[inline(always)]
    pub fn pe1ie(&self) -> PE1IE_R {
        PE1IE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PE2IE"]
    #[inline(always)]
    pub fn pe2ie(&self) -> PE2IE_R {
        PE2IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PE3IE"]
    #[inline(always)]
    pub fn pe3ie(&self) -> PE3IE_R {
        PE3IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PE4IE"]
    #[inline(always)]
    pub fn pe4ie(&self) -> PE4IE_R {
        PE4IE_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AE0IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae0ie(&mut self) -> AE0IE_W<DSI_IER0_SPEC, 0> {
        AE0IE_W::new(self)
    }
    #[doc = "Bit 1 - AE1IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae1ie(&mut self) -> AE1IE_W<DSI_IER0_SPEC, 1> {
        AE1IE_W::new(self)
    }
    #[doc = "Bit 2 - AE2IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae2ie(&mut self) -> AE2IE_W<DSI_IER0_SPEC, 2> {
        AE2IE_W::new(self)
    }
    #[doc = "Bit 3 - AE3IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae3ie(&mut self) -> AE3IE_W<DSI_IER0_SPEC, 3> {
        AE3IE_W::new(self)
    }
    #[doc = "Bit 4 - AE4IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae4ie(&mut self) -> AE4IE_W<DSI_IER0_SPEC, 4> {
        AE4IE_W::new(self)
    }
    #[doc = "Bit 5 - AE5IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae5ie(&mut self) -> AE5IE_W<DSI_IER0_SPEC, 5> {
        AE5IE_W::new(self)
    }
    #[doc = "Bit 6 - AE6IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae6ie(&mut self) -> AE6IE_W<DSI_IER0_SPEC, 6> {
        AE6IE_W::new(self)
    }
    #[doc = "Bit 7 - AE7IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae7ie(&mut self) -> AE7IE_W<DSI_IER0_SPEC, 7> {
        AE7IE_W::new(self)
    }
    #[doc = "Bit 8 - AE8IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae8ie(&mut self) -> AE8IE_W<DSI_IER0_SPEC, 8> {
        AE8IE_W::new(self)
    }
    #[doc = "Bit 9 - AE9IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae9ie(&mut self) -> AE9IE_W<DSI_IER0_SPEC, 9> {
        AE9IE_W::new(self)
    }
    #[doc = "Bit 10 - AE10IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae10ie(&mut self) -> AE10IE_W<DSI_IER0_SPEC, 10> {
        AE10IE_W::new(self)
    }
    #[doc = "Bit 11 - AE11IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae11ie(&mut self) -> AE11IE_W<DSI_IER0_SPEC, 11> {
        AE11IE_W::new(self)
    }
    #[doc = "Bit 12 - AE12IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae12ie(&mut self) -> AE12IE_W<DSI_IER0_SPEC, 12> {
        AE12IE_W::new(self)
    }
    #[doc = "Bit 13 - AE13IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae13ie(&mut self) -> AE13IE_W<DSI_IER0_SPEC, 13> {
        AE13IE_W::new(self)
    }
    #[doc = "Bit 14 - AE14IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae14ie(&mut self) -> AE14IE_W<DSI_IER0_SPEC, 14> {
        AE14IE_W::new(self)
    }
    #[doc = "Bit 15 - AE15IE"]
    #[inline(always)]
    #[must_use]
    pub fn ae15ie(&mut self) -> AE15IE_W<DSI_IER0_SPEC, 15> {
        AE15IE_W::new(self)
    }
    #[doc = "Bit 16 - PE0IE"]
    #[inline(always)]
    #[must_use]
    pub fn pe0ie(&mut self) -> PE0IE_W<DSI_IER0_SPEC, 16> {
        PE0IE_W::new(self)
    }
    #[doc = "Bit 17 - PE1IE"]
    #[inline(always)]
    #[must_use]
    pub fn pe1ie(&mut self) -> PE1IE_W<DSI_IER0_SPEC, 17> {
        PE1IE_W::new(self)
    }
    #[doc = "Bit 18 - PE2IE"]
    #[inline(always)]
    #[must_use]
    pub fn pe2ie(&mut self) -> PE2IE_W<DSI_IER0_SPEC, 18> {
        PE2IE_W::new(self)
    }
    #[doc = "Bit 19 - PE3IE"]
    #[inline(always)]
    #[must_use]
    pub fn pe3ie(&mut self) -> PE3IE_W<DSI_IER0_SPEC, 19> {
        PE3IE_W::new(self)
    }
    #[doc = "Bit 20 - PE4IE"]
    #[inline(always)]
    #[must_use]
    pub fn pe4ie(&mut self) -> PE4IE_W<DSI_IER0_SPEC, 20> {
        PE4IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DSI Host interrupt enable register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_ier0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsi_ier0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_IER0_SPEC;
impl crate::RegisterSpec for DSI_IER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ier0::R`](R) reader structure"]
impl crate::Readable for DSI_IER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_ier0::W`](W) writer structure"]
impl crate::Writable for DSI_IER0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSI_IER0 to value 0"]
impl crate::Resettable for DSI_IER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
