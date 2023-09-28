#[doc = "Register `L1WHPCR` reader"]
pub type R = crate::R<L1WHPCR_SPEC>;
#[doc = "Register `L1WHPCR` writer"]
pub type W = crate::W<L1WHPCR_SPEC>;
#[doc = "Field `WHSTPOS` reader - Window Horizontal Start Position"]
pub type WHSTPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSTPOS` writer - Window Horizontal Start Position"]
pub type WHSTPOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `WHSPPOS` reader - Window Horizontal Stop Position"]
pub type WHSPPOS_R = crate::FieldReader<u16>;
#[doc = "Field `WHSPPOS` writer - Window Horizontal Stop Position"]
pub type WHSPPOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Window Horizontal Start Position"]
    #[inline(always)]
    #[must_use]
    pub fn whstpos(&mut self) -> WHSTPOS_W<L1WHPCR_SPEC, 0> {
        WHSTPOS_W::new(self)
    }
    #[doc = "Bits 16:27 - Window Horizontal Stop Position"]
    #[inline(always)]
    #[must_use]
    pub fn whsppos(&mut self) -> WHSPPOS_W<L1WHPCR_SPEC, 16> {
        WHSPPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Layerx Window Horizontal Position Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1whpcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1whpcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1WHPCR_SPEC;
impl crate::RegisterSpec for L1WHPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1whpcr::R`](R) reader structure"]
impl crate::Readable for L1WHPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1whpcr::W`](W) writer structure"]
impl crate::Writable for L1WHPCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1WHPCR to value 0"]
impl crate::Resettable for L1WHPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
