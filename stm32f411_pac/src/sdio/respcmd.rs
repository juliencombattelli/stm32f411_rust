#[doc = "Register `RESPCMD` reader"]
pub type R = crate::R<RESPCMD_SPEC>;
#[doc = "Field `RESPCMD` reader - Response command index"]
pub type RESPCMD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index"]
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "command response register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respcmd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPCMD_SPEC;
impl crate::RegisterSpec for RESPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmd::R`](R) reader structure"]
impl crate::Readable for RESPCMD_SPEC {}
#[doc = "`reset()` method sets RESPCMD to value 0"]
impl crate::Resettable for RESPCMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
