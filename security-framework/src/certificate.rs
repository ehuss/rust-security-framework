use core_foundation_sys::base::{CFRelease, kCFAllocatorDefault};
use core_foundation::base::TCFType;
use core_foundation::data::CFData;
use core_foundation::string::CFString;
use security_framework_sys::base::{errSecParam, SecCertificateRef};
use security_framework_sys::certificate::*;
use std::mem;

use ErrorNew;
use base::{Error, Result};

#[derive(Debug)] // FIXME
pub struct SecCertificate(SecCertificateRef);

impl Drop for SecCertificate {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0 as *mut _); }
    }
}

impl SecCertificate {
    pub fn from_der(der_data: &[u8]) -> Result<SecCertificate> {
        let der_data = CFData::from_buffer(der_data);
        unsafe {
            let certificate = SecCertificateCreateWithData(kCFAllocatorDefault,
                                                           der_data.as_concrete_TypeRef());
            if certificate.is_null() {
                Err(Error::new(errSecParam))
            } else {
                Ok(SecCertificate::wrap_under_create_rule(certificate))
            }
        }
    }

    pub fn to_der(&self) -> CFData {
        unsafe {
            let der_data = SecCertificateCopyData(self.0);
            CFData::wrap_under_create_rule(der_data)
        }
    }

    pub fn subject_summary(&self) -> CFString {
        unsafe {
            let summary = SecCertificateCopySubjectSummary(self.0);
            CFString::wrap_under_create_rule(summary)
        }
    }
}

impl_TCFType!(SecCertificate, SecCertificateRef, SecCertificateGetTypeID);

#[cfg(test)]
mod test {
    use test::certificate;

    #[test]
    fn subject_summary() {
        let cert = certificate();
        assert_eq!("foobar.com", cert.subject_summary().to_string());
    }
}
