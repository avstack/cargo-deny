use crate::{
    diag::{CfgCoord, Diag, Diagnostic, Label, Severity},
    Krate,
};

pub(crate) struct Unlicensed<'a> {
    pub(crate) severity: Severity,
    pub(crate) krate: &'a Krate,
    pub(crate) breadcrumbs: Vec<Label>,
}

impl<'a> From<Unlicensed<'a>> for Diag {
    fn from(u: Unlicensed<'a>) -> Self {
        Diagnostic::new(u.severity)
            .with_message(format!("{} is unlicensed", u.krate))
            .with_code("L003")
            .with_labels(u.breadcrumbs)
            .into()
    }
}

pub(crate) struct SkippedPrivateWorkspaceCrate<'a> {
    pub(crate) krate: &'a Krate,
}

impl<'a> From<SkippedPrivateWorkspaceCrate<'a>> for Diag {
    fn from(spwc: SkippedPrivateWorkspaceCrate<'a>) -> Self {
        Diagnostic::new(Severity::Help)
            .with_message(format!("skipping private workspace crate '{}'", spwc.krate))
            .with_code("L004")
            .into()
    }
}

pub(crate) struct UnmatchedLicenseException {
    pub(crate) license_exc_cfg: CfgCoord,
}

impl From<UnmatchedLicenseException> for Diag {
    fn from(ule: UnmatchedLicenseException) -> Self {
        Diagnostic::new(Severity::Warning)
            .with_message("license exception was not encountered")
            .with_code("L005")
            .with_labels(vec![ule
                .license_exc_cfg
                .into_label()
                .with_message("unmatched license exception")])
            .into()
    }
}

pub(crate) struct UnmatchedLicenseAllowance {
    pub(crate) severity: Severity,
    pub(crate) allowed_license_cfg: CfgCoord,
}

impl From<UnmatchedLicenseAllowance> for Diag {
    fn from(ula: UnmatchedLicenseAllowance) -> Self {
        Diagnostic::new(ula.severity)
            .with_message("license was not encountered")
            .with_code("L006")
            .with_labels(vec![ula
                .allowed_license_cfg
                .into_label()
                .with_message("unmatched license allowance")])
            .into()
    }
}

pub(crate) struct MissingClarificationFile<'a> {
    pub(crate) expected: &'a crate::cfg::Spanned<std::path::PathBuf>,
    pub(crate) cfg_file_id: crate::diag::FileId,
}

impl<'a> From<MissingClarificationFile<'a>> for Label {
    fn from(mcf: MissingClarificationFile<'a>) -> Self {
        Label::secondary(mcf.cfg_file_id, mcf.expected.span.clone())
            .with_message("unable to locate specified license file")
    }
}
