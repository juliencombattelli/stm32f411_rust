#[doc = "Register `DTXFSTS0` reader"]
pub type R = crate::R<DTXFSTS0_SPEC>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS0_SPEC;
impl crate::RegisterSpec for DTXFSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts0::R`](R) reader structure"]
impl crate::Readable for DTXFSTS0_SPEC {}
#[doc = "`reset()` method sets DTXFSTS0 to value 0"]
impl crate::Resettable for DTXFSTS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
