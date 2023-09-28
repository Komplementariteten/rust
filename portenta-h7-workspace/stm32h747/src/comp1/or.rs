#[doc = "Register `OR` reader"]
pub type R = crate::R<OR_SPEC>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OR_SPEC>;
#[doc = "Field `AFOP` reader - Selection of source for alternate function of output ports"]
pub type AFOP_R = crate::FieldReader<u16>;
#[doc = "Field `AFOP` writer - Selection of source for alternate function of output ports"]
pub type AFOP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `OR` reader - Option Register"]
pub type OR_R = crate::FieldReader<u32>;
#[doc = "Field `OR` writer - Option Register"]
pub type OR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 21, O, u32>;
impl R {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    pub fn afop(&self) -> AFOP_R {
        AFOP_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    pub fn or(&self) -> OR_R {
        OR_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - Selection of source for alternate function of output ports"]
    #[inline(always)]
    #[must_use]
    pub fn afop(&mut self) -> AFOP_W<OR_SPEC, 0> {
        AFOP_W::new(self)
    }
    #[doc = "Bits 11:31 - Option Register"]
    #[inline(always)]
    #[must_use]
    pub fn or(&mut self) -> OR_W<OR_SPEC, 11> {
        OR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Comparator option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
