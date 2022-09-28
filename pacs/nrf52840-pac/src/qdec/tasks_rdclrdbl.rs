#[doc = "Register `TASKS_RDCLRDBL` writer"]
pub struct W(crate::W<TASKS_RDCLRDBL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_RDCLRDBL_SPEC>;
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
impl From<crate::W<TASKS_RDCLRDBL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_RDCLRDBL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TASKS_RDCLRDBL` writer - "]
pub type TASKS_RDCLRDBL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_RDCLRDBL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tasks_rdclrdbl(&mut self) -> TASKS_RDCLRDBL_W<0> {
        TASKS_RDCLRDBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read and clear ACCDBL\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_rdclrdbl](index.html) module"]
pub struct TASKS_RDCLRDBL_SPEC;
impl crate::RegisterSpec for TASKS_RDCLRDBL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_rdclrdbl::W](W) writer structure"]
impl crate::Writable for TASKS_RDCLRDBL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_RDCLRDBL to value 0"]
impl crate::Resettable for TASKS_RDCLRDBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
