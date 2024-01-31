#[doc = "Register `RESP1` reader"]
pub type R = crate::R<RESP1_SPEC>;
#[doc = "Field `CARDSTATUS1` reader - Card Status"]
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Card Status"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
#[doc = "response 1..4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESP1_SPEC;
impl crate::RegisterSpec for RESP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp1::R`](R) reader structure"]
impl crate::Readable for RESP1_SPEC {}
#[doc = "`reset()` method sets RESP1 to value 0"]
impl crate::Resettable for RESP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
