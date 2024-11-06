#[doc = "Register `TSSSR` reader"]
pub type R = crate::R<TSSSR_SPEC>;
#[doc = "Field `SS` reader - Sub second value"]
pub type SS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Sub second value"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "timestamp sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSSSR_SPEC;
impl crate::RegisterSpec for TSSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsssr::R`](R) reader structure"]
impl crate::Readable for TSSSR_SPEC {}
#[doc = "`reset()` method sets TSSSR to value 0"]
impl crate::Resettable for TSSSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
