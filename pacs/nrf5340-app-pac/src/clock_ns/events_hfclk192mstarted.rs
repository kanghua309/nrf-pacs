#[doc = "Register `EVENTS_HFCLK192MSTARTED` reader"]
pub struct R(crate::R<EVENTS_HFCLK192MSTARTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_HFCLK192MSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_HFCLK192MSTARTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_HFCLK192MSTARTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_HFCLK192MSTARTED` writer"]
pub struct W(crate::W<EVENTS_HFCLK192MSTARTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_HFCLK192MSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EVENTS_HFCLK192MSTARTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_HFCLK192MSTARTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_HFCLK192MSTARTED` reader - HFCLK192M source started"]
pub type EVENTS_HFCLK192MSTARTED_R = crate::BitReader<EVENTS_HFCLK192MSTARTED_A>;
#[doc = "HFCLK192M source started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_HFCLK192MSTARTED_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_HFCLK192MSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_HFCLK192MSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_HFCLK192MSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_HFCLK192MSTARTED_A {
        match self.bits {
            false => EVENTS_HFCLK192MSTARTED_A::NOT_GENERATED,
            true => EVENTS_HFCLK192MSTARTED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_HFCLK192MSTARTED_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_HFCLK192MSTARTED_A::GENERATED
    }
}
#[doc = "Field `EVENTS_HFCLK192MSTARTED` writer - HFCLK192M source started"]
pub type EVENTS_HFCLK192MSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_HFCLK192MSTARTED_SPEC, EVENTS_HFCLK192MSTARTED_A, O>;
impl<'a, const O: u8> EVENTS_HFCLK192MSTARTED_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_HFCLK192MSTARTED_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_HFCLK192MSTARTED_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - HFCLK192M source started"]
    #[inline(always)]
    pub fn events_hfclk192mstarted(&self) -> EVENTS_HFCLK192MSTARTED_R {
        EVENTS_HFCLK192MSTARTED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFCLK192M source started"]
    #[inline(always)]
    pub fn events_hfclk192mstarted(&mut self) -> EVENTS_HFCLK192MSTARTED_W<0> {
        EVENTS_HFCLK192MSTARTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFCLK192M source started\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_hfclk192mstarted](index.html) module"]
pub struct EVENTS_HFCLK192MSTARTED_SPEC;
impl crate::RegisterSpec for EVENTS_HFCLK192MSTARTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_hfclk192mstarted::R](R) reader structure"]
impl crate::Readable for EVENTS_HFCLK192MSTARTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_hfclk192mstarted::W](W) writer structure"]
impl crate::Writable for EVENTS_HFCLK192MSTARTED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_HFCLK192MSTARTED to value 0"]
impl crate::Resettable for EVENTS_HFCLK192MSTARTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
